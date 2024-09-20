use lazy_static::lazy_static;
use sqlx::SqlitePool;
use std::{path::PathBuf, str::FromStr};
use tauri::{async_runtime, path::BaseDirectory, Manager};
use tauri_plugin_fs::FsExt;
use tokio::sync::Mutex;
use tracing::{span, Level};

mod commands {
    pub mod assistant;
    pub mod crud;
    pub mod history;
}

mod ollama {
    pub mod ai;
    pub mod history;
}

mod database {
    pub mod assistants;
    pub mod configs;
    pub mod history;
    pub mod schemas;
}

#[derive(Default)]
struct AppState {
    pub db: Option<SqlitePool>,
    pub ai: Option<ollama::ai::Ai>,
}

lazy_static! {
    static ref DB_PATH: Mutex<PathBuf> = Mutex::new(PathBuf::new());
    static ref HISTORY_CACHE: Mutex<PathBuf> = Mutex::new(PathBuf::new());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();
    let _ = span!(Level::INFO, "Phallax");
    let state = AppState::default();
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .manage(Mutex::new(state))
        .invoke_handler(tauri::generate_handler![
            commands::assistant::completion,
            commands::assistant::get_chat_history,
            commands::assistant::init_ai,
            commands::history::load_history,
            commands::history::save_history,
            commands::crud::connect_to_database,
            commands::crud::insert_assistant,
            commands::crud::get_assistants,
            commands::crud::get_assistant_by_id,
            commands::crud::delete_assistant,
            commands::crud::update_assistant,
            commands::crud::insert_config,
            commands::crud::get_configs,
            commands::crud::get_config_by_id,
            commands::crud::delete_config,
            commands::crud::update_config,
            commands::crud::insert_history,
            commands::crud::get_history,
            commands::crud::get_history_by_id,
            commands::crud::delete_history,
            commands::crud::update_history,
        ])
        .setup(|app| {
            let db_path = app
                .path()
                .resolve("sqlite.db", BaseDirectory::AppData)
                .expect("Could not resolve path");
            let history_cache = app
                .path()
                .resolve(
                    PathBuf::from_str("phallax").unwrap().join("history"),
                    BaseDirectory::AppCache,
                )
                .expect("Could not resolve the cache dir.");

            let scope = app.fs_scope();
            scope.allow_file(&db_path);
            scope.allow_directory(&history_cache, true);

            if !db_path.exists() {
                let _ = std::fs::File::create(&db_path).expect("Could not create database file");
            }

            let connection_string = format!("sqlite:{}", db_path.to_string_lossy());
            tracing::info!("Connecting to database {}", &connection_string);
            async_runtime::block_on(async {
                let mut db_path_global = DB_PATH.lock().await;
                *db_path_global = db_path.clone();

                let mut history_cache_global = HISTORY_CACHE.lock().await;
                *history_cache_global = history_cache.clone();

                let pool = SqlitePool::connect(&connection_string)
                    .await
                    .expect("Couldn't establish connection to the database.");
                sqlx::migrate!("./migrations")
                    .run(&pool)
                    .await
                    .expect("Failed to migrate database");
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
