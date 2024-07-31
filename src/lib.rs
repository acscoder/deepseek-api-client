use optional_default::OptionalDefault;
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::format;
use std::future::Future;
use std::pin::Pin;
use std::{boxed::Box, rc::Rc};

use anyhow::{anyhow, Result};
use serde_json::{json, Value};
pub static CHAT_COMPLETION_API_URL: &str = "https://api.deepseek.com/chat/completions";
pub static DEEPSEEK_MODEL_CHAT: &str = "deepseek-chat";
pub static DEEPSEEK_MODEL_CODER: &str = "deepseek-coder";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}
#[derive(Serialize, Deserialize, OptionalDefault, Debug)]
pub struct FunctionalCallObject {
    r#type: String,
    function: FunctionalCallObjectSingle,
}
#[derive(Serialize, Deserialize, OptionalDefault, Debug)]
pub struct FunctionalCallObjectSingle {
    name: String,
    description: String,
    parameters: Value,
}
impl FunctionalCallObject {
    fn new(fname: &str, fdesc: &str, parameters: Value) -> Self {
        Self {
            r#type: "function".to_owned(),
            function: FunctionalCallObjectSingle {
                name: fname.to_owned(),
                description: fdesc.to_owned(),
                parameters,
            },
        }
    }
}
#[derive(Serialize, Deserialize, OptionalDefault, Debug)]
pub struct LogprobsObject {
    token: String,
    logprob: f32,
    #[optional(default = None)]
    bytes: Option<Vec<i32>>,
}
#[derive(Serialize, Deserialize, OptionalDefault, Debug)]
pub struct TopLogprobsObject {
    token: String,
    logprob: f32,
    #[optional(default = None)]
    bytes: Option<Vec<i32>>,
    top_logprobs: Vec<LogprobsObject>,
}
#[derive(Serialize, Deserialize, OptionalDefault, Debug)]
pub struct LogprobsContent {
    #[optional(default = None)]
    content: Option<Vec<TopLogprobsObject>>,
}
#[derive(Serialize, Deserialize, OptionalDefault, Clone, Debug)]
pub struct ChoiceObjectMessage {
    #[optional(default = "assistant".to_owned())]
    pub role: String,
    pub content: Option<String>,
}
#[derive(Serialize, Deserialize, OptionalDefault, Debug)]
pub struct ChoiceObject {
    #[optional(default = None)]
    finish_reason: Option<String>,
    index: i32,
    #[optional(default = None)]
    logprobs: Option<LogprobsContent>,
    message: ChoiceObjectMessage,
}
#[derive(Serialize, Deserialize, OptionalDefault, Debug)]
struct Usage {
    completion_tokens: i32,
    prompt_tokens: i32,
    total_tokens: i32,
}
#[derive(Serialize, Deserialize, OptionalDefault, Debug)]
struct ResponseFormatType {
    r#type: String,
}
#[derive(Serialize, OptionalDefault, Debug)]
pub struct RequestChat {
    messages: Vec<Message>,
    model: String,
    #[optional(default = 0.0)]
    frequency_penalty: f32,
    #[optional(default = 2048)]
    max_tokens: usize,
    #[optional(default = 0.0)]
    presence_penalty: f32,
    #[optional(default = ResponseFormatType{r#type: "text".to_owned()} )]
    response_format: ResponseFormatType,
    #[optional(default = None)]
    stop: Option<String>,
    #[optional(default = false)]
    stream: bool,
    #[optional(default = None)]
    stream_options: Option<String>,
    #[optional(default = 1.0)]
    temperature: f32,
    #[optional(default = 1.0)]
    top_p: f32,
    #[optional(default = None)]
    tools: Option<Vec<FunctionalCallObject>>,
    #[optional(default = "none".to_owned())]
    tool_choice: String,
    #[optional(default = false)]
    logprobs: bool,
    #[optional(default = None)]
    top_logprobs: Option<i32>,
}

#[derive(Serialize, Deserialize, OptionalDefault, Debug)]
pub struct ChatResponses {
    id: String,
    object: String,
    created: i64,
    model: String,
    system_fingerprint: String,
    choices: Vec<ChoiceObject>,
    #[optional(default = None)]
    usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, OptionalDefault, Debug)]
pub struct ChatResponsesStream {
    id: String,
    object: String,
    created: i64,
    model: String,
    system_fingerprint: String,
    choices: Vec<ChoiceObjectChunk>,
    #[optional(default = None)]
    usage: Option<Usage>,
}
#[derive(Serialize, Deserialize, OptionalDefault, Debug)]
pub struct ChoiceObjectChunk {
    #[optional(default = None)]
    finish_reason: Option<String>,
    index: i32,
    #[optional(default = None)]
    logprobs: Option<LogprobsContent>,
    delta: Value,
}
pub fn chat_DeepSeek_LLM_stream(
    mut params: RequestChat,
    api_key: &str,
) -> Box<
    dyn FnMut(
        Vec<Message>,
    ) -> Pin<Box<dyn Future<Output = Result<reqwest::Response, reqwest::Error>>>>,
> {
    let api_key_rc = Rc::new(api_key.to_owned());
    let c =  move  |messages: Vec<Message>| -> Pin<Box<dyn Future<Output = Result<reqwest::Response,reqwest::Error> >>> {
        params.messages = messages;
        let params_json =  serde_json::to_string(&params).unwrap();
        let client = reqwest::Client::new();
        let api_key = api_key_rc.clone();
        let req = client.post(CHAT_COMPLETION_API_URL)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key.to_string()))
        .body(params_json)
        .send();
        Box::pin(req) 
    };
    Box::new(c)
}
pub fn chat_deepSeek_LLM_synchornous(
    mut params: RequestChat,
    api_key: &str,
) -> Box<dyn FnMut(Vec<Message>) -> Result<ChatResponses>> {
    let api_key_rc = Rc::new(api_key.to_owned());
    let c = move |messages: Vec<Message>| -> Result<ChatResponses> {
        params.messages = messages;
        let params_json = serde_json::to_string(&params).unwrap();
        let api_key = api_key_rc.clone();
        let client = reqwest::blocking::Client::new();
        let req = client
            .post(CHAT_COMPLETION_API_URL)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key.to_string()))
            .body(params_json)
            .send();

        if let Ok(req) = req {
            let s = req.text().unwrap();
            let data = serde_json::from_str(&s);
            if let Ok(data) = data {
                return Ok(data);
            }
            return Err(anyhow!("Parse error {:?}", data));
        }
        Err(anyhow!("Can't connect to API"))
    };
    Box::new(c)
}

pub fn chat_DeepSeek_LLM(
    mut params: RequestChat,
    api_key: &str,
) -> Box<dyn FnMut(Vec<Message>) -> Pin<Box<dyn Future<Output = Result<ChatResponses>>>>> {
    let api_key_rc = Rc::new(api_key.to_owned());
    let f = move |messages: Vec<Message>| -> Pin<Box<dyn Future<Output = Result<ChatResponses>>>> {
        params.messages = messages;
        let is_stream = params.stream;
        let params_json = serde_json::to_string(&params).unwrap();

        let api_key = api_key_rc.clone();
        let c = async move {
            let client = reqwest::Client::new();
            let req = curl_post_request(
                &client,
                CHAT_COMPLETION_API_URL,
                params_json,
                api_key.to_string().as_str(),
            );
            if let Ok(req) = req {
                let res = client.execute(req);

                if let Ok(r) = res.await {
                    let s = r.text().await;

                    if let Ok(s) = s {
                        if is_stream {
                            let data = string_to_ChatResponses(&s);
                            Ok(data)
                        } else {
                            let data = serde_json::from_str(&s);
                            if data.is_ok() {
                                let d: ChatResponses = data.unwrap();
                                Ok(d)
                            } else {
                                Err(anyhow!("Parse error {:?}", data))
                            }
                        }
                    } else {
                        Err(anyhow!("Result response {:?}", s))
                    }
                } else {
                    Err(anyhow!("Can't connect to API"))
                }
            } else {
                Err(anyhow!("Request {:?}", req))
            }
        };
        Box::pin(c)
    };
    Box::new(f)
}
pub fn get_response_text(d: &ChatResponses, ind: usize) -> Option<String> {
    let response_index = d.choices.get(ind);
    if let Some(response_index) = response_index {
        response_index.message.content.clone()
    } else {
        None
    }
}
fn string_to_ChatResponses(s: &str) -> ChatResponses {
    let st = s.split("\n\n");
    let fold_init: ChatResponses = ChatResponses!( id: "".to_owned(),
                                object: "".to_owned(),
                                created: 0,
                                model: "".to_owned(),
                                system_fingerprint: "".to_owned(),
                                choices: vec![]);

    let data: ChatResponses = st.filter_map(|item|{
                                let sj = item.strip_prefix("data: ").unwrap_or(""); 
                                let dt = serde_json::from_str::<ChatResponsesStream>(sj).ok();
                                dt
                            }).fold(fold_init,|mut acc,item|{
                                if acc.choices.is_empty(){
                                    acc.id = item.id;
                                    acc.object = item.object;
                                    acc.created = item.created;
                                    acc.model = item.model;
                                    acc.system_fingerprint = item.system_fingerprint;
                                    let choice = item.choices.get(0).unwrap().delta.as_object().unwrap().get("content").unwrap().as_str().unwrap_or("").to_owned();
                                    acc.choices = vec![ChoiceObject!(finish_reason: None,index: item.choices.get(0).unwrap().index,logprobs: None,message: ChoiceObjectMessage!(content: Some(choice) ))];
                                }else{
                                    let choice = item.choices.get(0).unwrap().delta.as_object().unwrap().get("content").unwrap().as_str().unwrap_or("").to_owned();
                                    let acc_choices = acc.choices[0].message.content.clone().unwrap();
                                    acc.choices[0].message.content = Some(acc_choices+&choice);
                                } 
                                acc
                            });

    data
}
fn curl_post_request(
    client: &reqwest::Client,
    url: &str,
    params: String,
    api_key: &str,
) -> Result<reqwest::Request, reqwest::Error> {
    let req = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(params)
        .build();
    req
}

pub fn chat_completion(
    api_key: &str,
) -> Box<dyn FnMut(Vec<Message>) -> Pin<Box<dyn Future<Output = Result<ChatResponses>>>>> {
    let params = RequestChat! {
        model: DEEPSEEK_MODEL_CHAT.to_owned(),
        messages: vec![]
    };
    chat_DeepSeek_LLM(params, api_key)
}
pub fn code_completion(
    api_key: &str,
) -> Box<dyn FnMut(Vec<Message>) -> Pin<Box<dyn Future<Output = Result<ChatResponses>>>>> {
    let params = RequestChat! {
        model: DEEPSEEK_MODEL_CODER.to_owned(),
        stream:true,
        messages: vec![]
    };
    chat_DeepSeek_LLM(params, api_key)
}
pub fn llm_function_call(
    api_key: &str,
    tools: Vec<FunctionalCallObject>,
) -> Box<dyn FnMut(Vec<Message>) -> Pin<Box<dyn Future<Output = Result<ChatResponses>>>>> {
    let params = RequestChat! {
        model: DEEPSEEK_MODEL_CODER.to_owned(),
        messages: vec![],
        tools:Some(tools)
    };
    chat_DeepSeek_LLM(params, api_key)
}
pub fn chat_completion_stream(
    api_key: &str,
) -> Box<
    dyn FnMut(
        Vec<Message>,
    ) -> Pin<Box<dyn Future<Output = Result<reqwest::Response, reqwest::Error>>>>,
> {
    let params = RequestChat! {
        model: DEEPSEEK_MODEL_CHAT.to_owned(),
        stream:true,
        messages: vec![]
    };
    chat_DeepSeek_LLM_stream(params, api_key)
}
pub fn code_completion_stream(
    api_key: &str,
) -> Box<
    dyn FnMut(
        Vec<Message>,
    ) -> Pin<Box<dyn Future<Output = Result<reqwest::Response, reqwest::Error>>>>,
> {
    let params = RequestChat! {
        model: DEEPSEEK_MODEL_CODER.to_owned(),
        stream:true,
        messages: vec![]
    };
    chat_DeepSeek_LLM_stream(params, api_key)
}
pub fn llm_function_call_stream(
    api_key: &str,
    tools: Vec<FunctionalCallObject>,
) -> Box<
    dyn FnMut(
        Vec<Message>,
    ) -> Pin<Box<dyn Future<Output = Result<reqwest::Response, reqwest::Error>>>>,
> {
    let params = RequestChat! {
        model: DEEPSEEK_MODEL_CODER.to_owned(),
        stream:true,
        messages: vec![],
        tools:Some(tools)
    };
    chat_DeepSeek_LLM_stream(params, api_key)
}
pub fn chat_completion_sync(
    api_key: &str,
) -> Box<dyn FnMut(Vec<Message>) -> Result<ChatResponses>> {
    let params = RequestChat! {
        model: DEEPSEEK_MODEL_CHAT.to_owned(),
        messages: vec![]
    };
    chat_deepSeek_LLM_synchornous(params, api_key)
}
pub fn code_completion_sync(
    api_key: &str,
) -> Box<dyn FnMut(Vec<Message>) -> Result<ChatResponses>> {
    let params = RequestChat! {
        model: DEEPSEEK_MODEL_CODER.to_owned(),
        messages: vec![]
    };
    chat_deepSeek_LLM_synchornous(params, api_key)
}
pub fn llm_function_call_sync(
    api_key: &str,
    tools: Vec<FunctionalCallObject>,
) -> Box<dyn FnMut(Vec<Message>) -> Result<ChatResponses>> {
    let params = RequestChat! {
        model: DEEPSEEK_MODEL_CODER.to_owned(),
        messages: vec![],
        tools:Some(tools)
    };
    chat_deepSeek_LLM_synchornous(params, api_key)
}
#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;
    use tokio::runtime::Runtime;
    // replace by your API key
    pub static DEEPSEEK_API_KEY: &str = "sk-.......................";
    #[test]
    fn synchornous_completion_test() {
        
        let messages = vec![
            Message {
                role: "system".to_owned(),
                content: "You are a helpful assistant".to_owned(),
            },
            Message {
                role: "user".to_owned(),
                content: "Write Hello world in rust".to_owned(),
            },
        ];
        let mut llm = chat_completion_sync(DEEPSEEK_API_KEY);
        let res = llm(messages);
        let res_text = get_response_text(&res.unwrap(), 0);
        dbg!(res_text);
    }
    #[test]
    fn stream_completion_test() {
        let messages = vec![
            Message {
                role: "system".to_owned(),
                content: "You are a helpful assistant".to_owned(),
            },
            Message {
                role: "user".to_owned(),
                content: "Write Hello world in rust".to_owned(),
            },
        ];

        let mut llm = chat_completion_stream(DEEPSEEK_API_KEY);
        let rt = Runtime::new().unwrap();

        let dt = llm(messages);
        let _ = rt.block_on(async {
            let res = dt.await.unwrap();
            let mut stream = res.bytes_stream();
            while let Some(item) = stream.next().await {
                let item = item.unwrap();
                let s = match std::str::from_utf8(&item) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                let data = string_to_ChatResponses(s);
                let res = get_response_text(&data, 0).unwrap_or("".to_owned());
                println!("{}", res);
            }
        });
    }
    #[test]
    fn chat_completion_test() {
        let rt = Runtime::new().unwrap();
        let mut codeLLM = code_completion(DEEPSEEK_API_KEY);
        let messages = vec![
            Message {
                role: "system".to_owned(),
                content: "You are a helpful assistant".to_owned(),
            },
            Message {
                role: "user".to_owned(),
                content: "Write Hello world in rust".to_owned(),
            },
        ];
        let res = codeLLM(messages);
        let r = rt.block_on(async { get_response_text(&res.await.unwrap(), 0) });
        dbg!(&r);
        assert!(r.is_some());
    }
    #[test]
    fn function_call_test() {
        let rt = Runtime::new().unwrap();

        let tparam1 = json!({
            "type": "object",
            "required": ["location"],
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city and state, e.g. San Francisco, CA"
                }
            }
        });

        let t1 = FunctionalCallObject::new(
            "get_weather",
            "Get weather of an location, the user shoud supply a location first",
            tparam1,
        );

        let tools = vec![t1];
        let mut codeLLM = llm_function_call(DEEPSEEK_API_KEY, tools);

        let messages = vec![
            Message {
                role: "system".to_owned(),
                content: "You are a helpful assistant,your should reply in json format".to_owned(),
            },
            Message {
                role: "user".to_owned(),
                content: "How's the weather in Hangzhou?".to_owned(),
            },
        ];
        let res = codeLLM(messages);
        let r = rt.block_on(async {
            let d = res.await.unwrap();
            dbg!(&d);
            get_response_text(&d, 0)
        });
        dbg!(&r);
        assert!(r.is_some());
    }
}
