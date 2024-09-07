use crate::{
    database::schemas::{Assistant, Config},
    ollama::ai::Ai,
    AppState,
};

use ollama_rs::generation::chat::ChatMessage;
use tauri::State;

#[tauri::command(rename_all = "snake_case")]
pub async fn init_ai(
    state: State<'_, AppState>,
    assistant: Assistant,
    config: Config,
) -> Result<(), String> {
    let new_ai = Ai::new(assistant, config).map_err(|e| e.to_string())?;
    state.ai.lock().await.replace(new_ai);
    Ok(())
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn completion(
    state: State<'_, AppState>,
    model: String,
    prompt: String,
    assistant: Assistant,
    config: Config,
) -> Result<String, String> {
    let mut ai = state.ai.lock().await.take();

    if ai.is_none() {
        let new_ai = Ai::new(assistant, config).map_err(|e| e.to_string())?;
        state.ai.lock().await.replace(new_ai);
    }

    let ai = ai.as_mut().unwrap();

    let res = ai
        .completion(&model, &prompt)
        .await
        .map_err(|e| e.to_string())?;

    Ok(res)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_chat_history(state: State<'_, AppState>) -> Result<Vec<ChatMessage>, String> {
    let mut ai = state.ai.lock().await;
    if ai.is_none() {
        return Err("AI is not initialized".to_string());
    }
    let ai = ai.as_mut().unwrap();
    let res = ai.get_history().map_err(|e| e.to_string())?;
    Ok(res)
}
