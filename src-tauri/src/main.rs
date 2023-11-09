// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use dotenv::dotenv;
use reqwest::Body;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize)]
struct OpenAIChoices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    id: String,
    choices: Vec<OpenAIChoices>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    prompt: String,
    max_tokens: u16,
    result: Result<(), String>,
}

#[tauri::command]
async fn generate_query() -> OpenAIRequest {
    let preamble = "Generate a Sql code for the given statement";

    println!("{esc}c", esc = 27 as char);

    let result = match do_generate_query().await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    };

    let oai_request = OpenAIRequest {
        prompt: format!("{} {}", preamble, "Get employees data from employees table"),
        max_tokens: 1000,
        result,
    };

    oai_request
}

async fn do_generate_query() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.openai.com/v1/engines/text-davinci-001/completions";
    let preamble = "Generate a Sql code for the given statement";
    let oai_token: String = match env::var("OAI_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            eprintln!("Error: OAI_TOKEN not found in environment");
            return Ok(());
        }
    };
    let auth_header_val = format!("Bearer {}", oai_token);

    println!("{esc}c", esc = 27 as char);

    let oai_request = OpenAIRequest {
        prompt: format!("{} {}", preamble, "Get employees data from employees table"),
        max_tokens: 1000,
        result: Ok(()),
    };

    let body = Body::from(serde_json::to_vec(&oai_request)?);

    let client = reqwest::Client::new();

    let res = client
        .post(url)
        .body(body)
        .header("Authorization", auth_header_val)
        .header("Content-Type", "application/json")
        .send()
        .await;

    println!("res {:?}", res);

    res?;
    Ok(())
}

fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
