use crate::{
    database::{
        assistants, configs, history,
        schemas::{Assistant, Config, History},
    },
    AppState,
};
use tauri::Manager;

#[tauri::command(async, rename_all = "snake_case")]
pub async fn insert_assistant(app: tauri::AppHandle, assistant: Assistant) -> Result<i64, String> {
    let state = app.state::<AppState>();
    let pool = state.db.lock().await;
    let id = assistants::insert_assistant(&pool, assistant).await;

    match id {
        Ok(id) => Ok(id),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn get_assistant_by_id(app: tauri::AppHandle, id: i64) -> Result<Assistant, String> {
    let state = app.state::<AppState>();
    let pool = state.db.lock().await;
    let assistant = assistants::get_assistant_by_id(&pool, id).await;

    match assistant {
        Ok(assistant) => Ok(assistant),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn get_assistants(app: tauri::AppHandle) -> Result<Vec<Assistant>, String> {
    let state = app.state::<AppState>();
    let pool = state.db.lock().await;
    let assistants = assistants::get_assistants(&pool).await;
    match assistants {
        Ok(assistants) => Ok(assistants),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn update_assistant(app: tauri::AppHandle, assistant: Assistant) -> Result<(), String> {
    let state = app.state::<AppState>();

    let pool = state.db.lock().await;
    let result = assistants::update_assistant(&pool, assistant).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn delete_assistant(app: tauri::AppHandle, id: i64) -> Result<(), String> {
    let state = app.state::<AppState>();
    let pool = state.db.lock().await;
    let result = assistants::delete_assistant(&pool, id).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn insert_history(app: tauri::AppHandle, history: History) -> Result<i64, String> {
    let state = app.state::<AppState>();
    let pool = state.db.lock().await;
    let id = history::insert_history(&pool, history).await;
    match id {
        Ok(id) => Ok(id),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn get_history_by_id(app: tauri::AppHandle, id: i64) -> Result<History, String> {
    let state = app.state::<AppState>();
    let pool = state.db.lock().await;
    let history = history::get_history_by_id(&pool, id).await;
    match history {
        Ok(history) => Ok(history),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn get_history(app: tauri::AppHandle) -> Result<Vec<History>, String> {
    let state = app.state::<AppState>();
    let pool = state.db.lock().await;
    let histories = history::get_history(&pool).await;
    match histories {
        Ok(histories) => Ok(histories),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn update_history(app: tauri::AppHandle, history: History) -> Result<(), String> {
    let state = app.state::<AppState>();
    let pool = state.db.lock().await;
    let result = history::update_history(&pool, history).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn delete_history(app: tauri::AppHandle, id: i64) -> Result<(), String> {
    let state = app.state::<AppState>();
    let pool = state.db.lock().await;
    let result = history::delete_history(&pool, id).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn insert_config(app: tauri::AppHandle, config: Config) -> Result<i64, String> {
    let state = app.state::<AppState>();
    let pool = state.db.lock().await;
    let id = configs::insert_config(&pool, config).await;
    match id {
        Ok(id) => Ok(id),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn get_config_by_id(app: tauri::AppHandle, id: i64) -> Result<Config, String> {
    let state = app.state::<AppState>();
    let pool = state.db.lock().await;
    let config = configs::get_config_by_id(&pool, id).await;
    match config {
        Ok(config) => Ok(config),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn get_configs(app: tauri::AppHandle) -> Result<Vec<Config>, String> {
    let state = app.state::<AppState>();
    let pool = state.db.lock().await;
    let configs = configs::get_configs(&pool).await;
    match configs {
        Ok(configs) => Ok(configs),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn update_config(app: tauri::AppHandle, config: Config) -> Result<(), String> {
    let state = app.state::<AppState>();
    let pool = state.db.lock().await;
    let result = configs::update_config(&pool, config).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn delete_config(app: tauri::AppHandle, id: i64) -> Result<(), String> {
    let state = app.state::<AppState>();
    let pool = state.db.lock().await;
    let result = configs::delete_config(&pool, id).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}
