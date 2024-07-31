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
1. get llm function
```rust
let mut chat_complete = chat_completion_sync(api_key) ;
```
And you will get a function that take input vector of Message and give back Result of Response 
unwrap response result then take the first choice response text by function get_response_text
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
let res = llm(messages);
let res_text = get_response_text(&res.unwrap(), 0);
dbg!(res_text);
```

### Call acsynchronous function

### Call synchronous function stream