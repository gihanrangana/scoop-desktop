// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::AppHandle;
use scoop_lib::{github_api, scoop_check};

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

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            check_scoop_installed,
            install_scoop,
            fetch_json_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
