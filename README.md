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
3 . Do the same with function  `code_completion_sync` for code generation with `deepseek-coder` model 

### Call asynchronous function
1. Get llm by function `chat_completion`
```rust
let mut llm_completion = chat_completion(api_key) ;
```
2. It the same `chat_completion_sync` but it is async function that we can call with .await, i used tokio crate for async runtime
```rust
let rt = Runtime::new().unwrap();
```
3. Input vector of Message and get back Result then then take the first choice response text by function `get_response_text`
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
3. Do the same with function  `code_completion` for code generation with `deepseek-coder` model 

### Call asynchronous function stream