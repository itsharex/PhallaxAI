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
