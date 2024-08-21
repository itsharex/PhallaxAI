use sqlx::SqlitePool;
use std::{path::PathBuf, str::FromStr, sync::Arc};
use tauri::{async_runtime, path::BaseDirectory, Manager};
use tauri_plugin_fs::FsExt;
use tokio::sync::Mutex;
use tracing::{span, Level};

mod commands {
    pub mod assistant;
}

struct AppState {
    db: Mutex<SqlitePool>,
}

impl AppState {
    pub fn new(pool: SqlitePool) -> Arc<Self> {
        Arc::new(Self {
            db: Mutex::new(pool),
        })
    }
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
            scope.allow_directory(history_cache, true);

            if !db_path.exists() {
                let _ = std::fs::File::create(&db_path).expect("Could not create database file");
            }

            let connection_string = format!("sqlite:{}", db_path.to_string_lossy());
            tracing::info!("Connecting to database {}", &connection_string);

            async_runtime::block_on(async {
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
        .invoke_handler(tauri::generate_handler![commands::assistant::completion,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
