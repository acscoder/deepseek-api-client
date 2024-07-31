# Unofficial api client for Deepseek 
Sign up for an account on https://platform.deepseek.com/sign_in to get your API key.


```
[dependencies]
deepseek-api-client = {  path = "https://github.com/acscoder/deepseek-api-client.git" } 
```
### Get started
Load your API key in env var or any secret way 
```rust
use deepseek_api_client::*;
let api_key = std::env::var("DEEPSEEK_API_KEY").expect("$DEEPSEEK_API_KEY is not set");
```

### Call synchronous function
1. Get llm by function `chat_completion_sync`
```rust
let mut llm_completion = chat_completion_sync(api_key) ;
```
2. Then you will get a function `llm_completion` that take input vector of Message and get back Result of Response 
unwrap response result then take the first choice response text by function `get_response_text`
```rust
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
let res = llm_completion(messages);
let res_text = get_response_text(&res.unwrap(), 0);
dbg!(res_text);
```
3 . Do the same with function  `code_completion_sync` for code generation with `deepseek-coder` model and `llm_function_call_sync` for function calling

### Call asynchronous function
1. Get llm by function `chat_completion`
```rust
let mut llm_completion = chat_completion(api_key) ;
```
2. It the same `chat_completion_sync` but it is async function that we can call with .await, i used tokio crate for async runtime
```rust
let rt = Runtime::new().unwrap();
```
3. Input vector of Message and get back Result then take the first choice response text by function `get_response_text`
```rust
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
let res = llm_completion(messages);
let r = rt.block_on(async { get_response_text(&res.await.unwrap(), 0) });
dbg!(&r);
```
4. Do the same with function `code_completion` for code generation with `deepseek-coder` model and `llm_function_call` for function calling

### Call asynchronous function stream
1. Get llm by `chat_completion_stream`
```rust
let mut llm_completion = chat_completion_stream(api_key) ;
```
2. We have async function that take input vector of Message and get back stream of Response
```rust
let rt = Runtime::new().unwrap();
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
let response_result = llm_completion(messages);
let _ = rt.block_on(async {
            let res = response_result.await.unwrap();
            let mut stream = res.bytes_stream();
            while let Some(item) = stream.next().await {
                let item = item.unwrap();
                let s = match std::str::from_utf8(&item) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                let data = string_to_ChatResponses(s);
                let text = get_response_text(&data, 0).unwrap_or("".to_owned());
                println!("{}", text);
            }
        });
```
3. Do the same with function `code_completion_stream` for code generation with `deepseek-coder` model and `llm_function_call_stream` for function calling

