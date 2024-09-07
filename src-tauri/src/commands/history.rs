use crate::{database::schemas::History, ollama::history};
use ollama_rs::generation::chat::ChatMessage;

#[tauri::command(async, rename_all = "snake_case")]
pub async fn save_history(history: History, messages: Vec<ChatMessage>) -> Result<(), String> {
    history::save_history(history, messages)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn load_history(history: History) -> Result<Vec<ChatMessage>, String> {
    history::load_history(history)
        .await
        .map_err(|e| e.to_string())
}
