// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use dotenv::dotenv;
use reqwest::{header, Body};
use serde_derive::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize)]
struct OpenAIChoices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct OpenAIResponse {
    prompt: String,
    max_tokens: u16,
    result: Result<String, String>,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    prompt: String,
    max_tokens: u16,
}

#[tauri::command]
async fn generate_query(user_text: String) -> OpenAIResponse {
    let preamble = "Generate a Sql code for the given statement";

    println!("{esc}c", esc = 27 as char);

    let result = match do_generate_query(user_text).await {
        Ok(data) => Ok(data),
        Err(err) => Err(err.to_string()),
    };

    println!("oai_request: {:?}", result);

    let oai_request = OpenAIResponse {
        prompt: format!("{} {}", preamble, "Get employees data from employees table"),
        max_tokens: 1000,
        result,
    };

    oai_request
}

async fn do_generate_query(user_text: String) -> Result<String, Box<dyn Error>> {
    let url = "https://api.openai.com/v1/engines/text-davinci-001/completions";
    let preamble = "Generate a Sql code for the given statement";

    let oai_token = "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";
    let auth_header_val = format!("Bearer {}", oai_token);

    println!("{esc}c", esc = 27 as char);

    let oai_request = OpenAIRequest {
        prompt: format!("{} {}", preamble, user_text),
        max_tokens: 1000,
    };

    let body = Body::from(serde_json::to_vec(&oai_request)?);

    let client = reqwest::Client::new();

    let res = client
        .post(url)
        .body(body)
        .header(header::AUTHORIZATION, auth_header_val)
        .header(header::CONTENT_TYPE, "application/json")
        .send()
        .await?;

    if res.status().is_success() {
        let body_text = res.text().await?;
        println!("Response Body: {}", body_text);
        Ok(body_text)
    } else {
        // Provide an error message for non-successful status codes
        Err(format!("Request failed with status code: {}", res.status()).into())
    }
}

fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
