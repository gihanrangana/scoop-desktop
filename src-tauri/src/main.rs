// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use scoop_lib::{github_api, scoop_check};
use tauri::AppHandle;
use tauri_plugin_sql::{Migration, MigrationKind};

#[tauri::command]
async fn check_scoop_installed(app_handle: AppHandle) -> scoop_check::ScoopStatus {
    scoop_check::check_scoop_installed(app_handle).await
}

#[tauri::command]
async fn install_scoop(app_handle: AppHandle) -> Result<String, String> {
    scoop_check::install_scoop(app_handle).await
}

#[tauri::command]
async fn fetch_json_files() -> Result<github_api::ApiResponse, String> {
    github_api::fetch_json_files().await
}

#[tauri::command]
async fn get_available_buckets() -> Result<serde_json::Value, String> {
    github_api::get_available_buckets().await
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations(
                    "sqlite:app.db",
                    vec![Migration {
                        version: 1,
                        description: "create initial tables",
                        sql: "CREATE TABLE IF NOT EXISTS buckets (
                        id INTEGER PRIMARY KEY,
                        name TEXT NOT NULL UNIQUE,
                        description TEXT,
                        url TEXT,
                        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                    )",
                        kind: MigrationKind::Up,
                    }]
                )
                .add_migrations(
                    "sqlite:app.db",
                    vec![Migration {
                        version: 1,
                        description: "drop tables",
                        sql: "DROP TABLE IF EXISTS buckets",
                        kind: MigrationKind::Down,
                    }]
                )
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            check_scoop_installed,
            install_scoop,
            fetch_json_files,
            get_available_buckets
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
