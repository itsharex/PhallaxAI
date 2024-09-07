use commands::{assistant, crud, history};
use lazy_static::lazy_static;
use sqlx::SqlitePool;
use std::{path::PathBuf, str::FromStr, sync::Arc};
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

struct AppState {
    pub db: Mutex<SqlitePool>,
    pub ai: Mutex<Option<ollama::ai::Ai>>,
}

impl AppState {
    pub fn new(pool: SqlitePool) -> Arc<Self> {
        Arc::new(Self {
            db: Mutex::new(pool),
            ai: Mutex::new(None),
        })
    }
}

lazy_static! {
    static ref DB_PATH: Mutex<PathBuf> = Mutex::new(PathBuf::new());
    static ref HISTORY_CACHE: Mutex<PathBuf> = Mutex::new(PathBuf::new());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();
    let _ = span!(Level::INFO, "Phallax");
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
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
                let state = AppState::new(pool);
                app.manage(state);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            assistant::completion,
            assistant::get_chat_history,
            assistant::init_ai,
            history::load_history,
            history::save_history,
            crud::insert_assistant,
            crud::get_assistants,
            crud::get_assistant_by_id,
            crud::delete_assistant,
            crud::update_assistant,
            crud::insert_config,
            crud::get_configs,
            crud::get_config_by_id,
            crud::delete_config,
            crud::update_config,
            crud::insert_history,
            crud::get_history,
            crud::get_history_by_id,
            crud::delete_history,
            crud::update_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
