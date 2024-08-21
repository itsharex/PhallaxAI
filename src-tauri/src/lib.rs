use sqlx::SqlitePool;
use std::sync::Arc;
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
            db: Arc::new(Mutex::new(pool)),
        })
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = span!(Level::TRACE, "");
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let db_path = app
                .path()
                .resolve("phallax.db", BaseDirectory::AppData)
                .expect("Could not resolve path");
            let history_cache = app
                .path()
                .resolve("phallax/history", BaseDirectory::AppCache)
                .expect("Could not resolve the cache dir.");
            let scope = app.fs_scope();
            scope.allow_file(&db_path);
            scope.allow_directory(history_cache, true);

            async_runtime::spawn(async move || {
                let connection_string = format!("sqlite:///{}", db_path).as_str();
                tracing::info!("Connecting to database {:?}", connection_string);
                let pool = SqlitePool::connect(connection_string)
                    .await
                    .expect("Couldn't establish connection to the database.");
                sqlx::migrate!("migrations")
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
