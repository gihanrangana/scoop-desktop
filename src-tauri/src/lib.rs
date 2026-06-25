mod commands;
mod services;
mod utils;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_all_packages,
            get_packages_by_bucket,
            search_packages,
            get_package_info,
            get_installed_packages,
            check_if_installed,
            install_package,
            uninstall_package,
            get_buckets,
            get_recent_installed_packages,
            get_package_icon
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
