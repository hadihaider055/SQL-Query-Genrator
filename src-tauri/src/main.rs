// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
}

#[tauri::command]
async fn generate_query() -> String {}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
