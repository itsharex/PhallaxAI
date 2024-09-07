use crate::{
    database::{
        assistants, configs, history,
        schemas::{Assistant, Config, History},
    },
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

#[tauri::command(async, rename_all = "snake_case")]
pub async fn insert_history(state: State<'_, AppState>, history: History) -> Result<i64, String> {
    let pool = state.db.lock().await;
    let id = history::insert_history(&pool, history).await;
    match id {
        Ok(id) => Ok(id),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn get_history_by_id(state: State<'_, AppState>, id: i64) -> Result<History, String> {
    let pool = state.db.lock().await;
    let history = history::get_history_by_id(&pool, id).await;
    match history {
        Ok(history) => Ok(history),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn get_history(state: State<'_, AppState>) -> Result<Vec<History>, String> {
    let pool = state.db.lock().await;
    let histories = history::get_history(&pool).await;
    match histories {
        Ok(histories) => Ok(histories),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn update_history(state: State<'_, AppState>, history: History) -> Result<(), String> {
    let pool = state.db.lock().await;
    let result = history::update_history(&pool, history).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn delete_history(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let pool = state.db.lock().await;
    let result = history::delete_history(&pool, id).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn insert_config(state: State<'_, AppState>, config: Config) -> Result<i64, String> {
    let pool = state.db.lock().await;
    let id = configs::insert_config(&pool, config).await;
    match id {
        Ok(id) => Ok(id),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn get_config_by_id(state: State<'_, AppState>, id: i64) -> Result<Config, String> {
    let pool = state.db.lock().await;
    let config = configs::get_config_by_id(&pool, id).await;
    match config {
        Ok(config) => Ok(config),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn get_configs(state: State<'_, AppState>) -> Result<Vec<Config>, String> {
    let pool = state.db.lock().await;
    let configs = configs::get_configs(&pool).await;
    match configs {
        Ok(configs) => Ok(configs),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn update_config(state: State<'_, AppState>, config: Config) -> Result<(), String> {
    let pool = state.db.lock().await;
    let result = configs::update_config(&pool, config).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn delete_config(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let pool = state.db.lock().await;
    let result = configs::delete_config(&pool, id).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}
