# Unofficial api client for Deepseek 
Sign up for an account on https://platform.deepseek.com/sign_in to get your API key.

[dependencies]
deepseek-api-client = "0.1"

### Get started
Load your API key in env var or any secret way 

use deepseek_api_client::*;
let api_key = std::env::var("DEEPSEEK_API_KEY").expect("$DEEPSEEK_API_KEY is not set");

### Call synchronous function
chat_deepSeek_LLM_synchornous 

### Call acsynchronous function

### Call synchronous function stream