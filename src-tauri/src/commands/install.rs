use crate::utils::{
    get_apps_path, get_install_timestamp, get_installed_version, is_package_installed,
    read_installed_manifest,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;
use tauri::command;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
    pub bucket: String,
    pub install_path: String,
    pub installed_at: i64,
    pub description: String,
    pub homepage: Option<String>,
}

#[command]
pub async fn get_installed_packages() -> Result<Vec<InstalledPackage>, String> {
    let apps_path = get_apps_path()?;
    let mut packages = Vec::new();

    if let Ok(entries) = fs::read_dir(&apps_path) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                    if name == "scoop" {
                        continue;
                    }

                    let install_json = path.join("current").join("install.json");

                    if install_json.exists() {
                        if let Ok(content) = fs::read_to_string(&install_json) {
                            if let Ok(install_info) =
                                serde_json::from_str::<serde_json::Value>(&content)
                            {
                                let bucket = install_info["bucket"]
                                    .as_str()
                                    .unwrap_or("unknown")
                                    .to_string();

                                let manifest = read_installed_manifest(name).ok();

                                let version = manifest
                                    .as_ref()
                                    .map(|m| m.version.clone())
                                    .unwrap_or_else(|| {
                                        get_installed_version(name)
                                            .unwrap_or_else(|_| "unknown".to_string())
                                    });

                                let description = manifest
                                    .as_ref()
                                    .map(|m| m.description.clone())
                                    .unwrap_or_else(|| "unknown".to_string());

                                packages.push(InstalledPackage {
                                    name: name.to_string(),
                                    version: version.clone(),
                                    bucket,
                                    install_path: path.to_string_lossy().to_string(),
                                    installed_at: get_install_timestamp(name)?,
                                    description: description.to_string(),
                                    homepage: manifest.as_ref().and_then(|m| m.homepage.clone()),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(packages)
}

#[command]
pub async fn check_if_installed(name: String) -> Result<bool, String> {
    is_package_installed(&name)
}

#[command]
pub async fn install_package(name: String) -> Result<String, String> {
    let output = Command::new("scoop")
        .args(&["install", &name])
        .output()
        .map_err(|e| format!("Failed to install package: {}", e))?;

    if output.status.success() {
        Ok(format!("Successfully installed package: {}", name))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to install package: {}", stderr))
    }
}

#[command]
pub async fn uninstall_package(name: String) -> Result<String, String> {
    let output = Command::new("scoop")
        .args(&["uninstall", &name])
        .output()
        .map_err(|e| format!("Failed to uninstall package: {}", e))?;

    if output.status.success() {
        Ok(format!("Successfully uninstalled package: {}", name))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to uninstall package: {}", stderr))
    }
}

#[command]
pub async fn get_recent_installed_packages(
    limit: Option<usize>,
) -> Result<Vec<InstalledPackage>, String> {
    let mut packages = get_installed_packages().await?;

    packages.sort_by(|a, b| b.installed_at.cmp(&a.installed_at));

    let limit = limit.unwrap_or(5);
    packages.truncate(limit);

    Ok(packages)
}
