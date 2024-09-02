use crate::{
    database::{assistants, schemas::Assistant},
    AppState,
};
use tauri::State;

#[tauri::command(async, rename_all = "snake_case")]
pub async fn insert_assistant(
    state: State<'_, AppState>,
    assistant: Assistant,
) -> Result<i64, String> {
    let pool = state.db.lock().await;
    let id = assistants::insert_assistant(&pool, assistant).await;

    match id {
        Ok(id) => Ok(id),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn get_assistant_by_id(state: State<'_, AppState>, id: i64) -> Result<Assistant, String> {
    let pool = state.db.lock().await;
    let assistant = assistants::get_assistant_by_id(&pool, id).await;

    match assistant {
        Ok(assistant) => Ok(assistant),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn get_assistants(state: State<'_, AppState>) -> Result<Vec<Assistant>, String> {
    let pool = state.db.lock().await;
    let assistants = assistants::get_assistants(&pool).await;
    match assistants {
        Ok(assistants) => Ok(assistants),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn update_assistant(
    state: State<'_, AppState>,
    assistant: Assistant,
) -> Result<(), String> {
    let pool = state.db.lock().await;
    let result = assistants::update_assistant(&pool, assistant).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn delete_assistant(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let pool = state.db.lock().await;
    let result = assistants::delete_assistant(&pool, id).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}
