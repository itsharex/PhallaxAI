use crate::{
    database::{
        assistants, configs, history,
        schemas::{Assistant, Config, History},
    },
    AppState, DB_PATH,
};
use sqlx::SqlitePool;
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command(async, rename_all = "snake_case")]
pub async fn connect_to_database(state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let mut state = state.lock().await;

    state.db = Some(
        SqlitePool::connect(&DB_PATH.lock().await.as_path().to_string_lossy())
            .await
            .map_err(|err| err.to_string())?,
    );

    Ok(())
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn insert_assistant(
    state: State<'_, Mutex<AppState>>,
    assistant: Assistant,
) -> Result<i64, String> {
    let state = state.lock().await;
    let pool = state.db.as_ref().ok_or("Database not connected")?;
    let id = assistants::insert_assistant(pool, assistant).await;

    match id {
        Ok(id) => Ok(id),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn get_assistant_by_id(
    state: State<'_, Mutex<AppState>>,
    id: i64,
) -> Result<Assistant, String> {
    let state = state.lock().await;
    let pool = state.db.as_ref().ok_or("Database not connected")?;
    let assistant = assistants::get_assistant_by_id(pool, id).await;

    match assistant {
        Ok(assistant) => Ok(assistant),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn get_assistants(state: State<'_, Mutex<AppState>>) -> Result<Vec<Assistant>, String> {
    let state = state.lock().await;
    let pool = state.db.as_ref().ok_or("Database not connected")?;
    let assistants = assistants::get_assistants(pool).await;
    match assistants {
        Ok(assistants) => Ok(assistants),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn update_assistant(
    state: State<'_, Mutex<AppState>>,
    assistant: Assistant,
) -> Result<(), String> {
    let state = state.lock().await;
    let pool = state.db.as_ref().ok_or("Database not connected")?;
    let result = assistants::update_assistant(pool, assistant).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn delete_assistant(state: State<'_, Mutex<AppState>>, id: i64) -> Result<(), String> {
    let state = state.lock().await;
    let pool = state.db.as_ref().ok_or("Database not connected")?;
    let result = assistants::delete_assistant(pool, id).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn insert_history(
    state: State<'_, Mutex<AppState>>,
    history: History,
) -> Result<i64, String> {
    let state = state.lock().await;
    let pool = state.db.as_ref().ok_or("Database not connected")?;
    let id = history::insert_history(pool, history).await;
    match id {
        Ok(id) => Ok(id),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn get_history_by_id(
    state: State<'_, Mutex<AppState>>,
    id: i64,
) -> Result<History, String> {
    let state = state.lock().await;
    let pool = state.db.as_ref().ok_or("Database not connected")?;
    let history = history::get_history_by_id(pool, id).await;
    match history {
        Ok(history) => Ok(history),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn get_history(state: State<'_, Mutex<AppState>>) -> Result<Vec<History>, String> {
    let state = state.lock().await;
    let pool = state.db.as_ref().ok_or("Database not connected")?;
    let histories = history::get_history(pool).await;
    match histories {
        Ok(histories) => Ok(histories),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn update_history(
    state: State<'_, Mutex<AppState>>,
    history: History,
) -> Result<(), String> {
    let state = state.lock().await;
    let pool = state.db.as_ref().ok_or("Database not connected")?;
    let result = history::update_history(pool, history).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn delete_history(state: State<'_, Mutex<AppState>>, id: i64) -> Result<(), String> {
    let state = state.lock().await;
    let pool = state.db.as_ref().ok_or("Database not connected")?;
    let result = history::delete_history(pool, id).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn insert_config(
    state: State<'_, Mutex<AppState>>,
    config: Config,
) -> Result<i64, String> {
    let state = state.lock().await;
    let pool = state.db.as_ref().ok_or("Database not connected")?;
    let id = configs::insert_config(pool, config).await;
    match id {
        Ok(id) => Ok(id),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn get_config_by_id(
    state: State<'_, Mutex<AppState>>,
    id: i64,
) -> Result<Config, String> {
    let state = state.lock().await;
    let pool = state.db.as_ref().ok_or("Database not connected")?;
    let config = configs::get_config_by_id(pool, id).await;
    match config {
        Ok(config) => Ok(config),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn get_configs(state: State<'_, Mutex<AppState>>) -> Result<Vec<Config>, String> {
    let state = state.lock().await;
    let pool = state.db.as_ref().ok_or("Database not connected")?;
    let configs = configs::get_configs(pool).await;
    match configs {
        Ok(configs) => Ok(configs),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn update_config(
    state: State<'_, Mutex<AppState>>,
    config: Config,
) -> Result<(), String> {
    let state = state.lock().await;
    let pool = state.db.as_ref().ok_or("Database not connected")?;
    let result = configs::update_config(pool, config).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command(async, rename_all = "snake_case")]
pub async fn delete_config(state: State<'_, Mutex<AppState>>, id: i64) -> Result<(), String> {
    let state = state.lock().await;
    let pool = state.db.as_ref().ok_or("Database not connected")?;
    let result = configs::delete_config(pool, id).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}
