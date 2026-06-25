use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheMetadata {
    pub source: String,
    pub cached_at: u64,
    pub ttl_seconds: u64,
}

impl CacheMetadata {
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now > self.cached_at + self.ttl_seconds
    }
}

pub fn icon_cache_path(app: &tauri::AppHandle, pkg_name: &str) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_cache_dir()
        .map_err(|e| e.to_string())?
        .join("package-icons");

    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    Ok(dir.join(format!("{}.png", pkg_name)))
}

pub fn metadata_cache_path(app: &tauri::AppHandle, pkg_name: &str) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_cache_dir()
        .map_err(|e| e.to_string())?
        .join("package-icons");

    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    Ok(dir.join(format!("{}.json", pkg_name)))
}

pub fn read_cache_icon(app: &tauri::AppHandle, pkg_name: &str) -> Option<Vec<u8>> {
    // let path = icon_cache_path(app, pkg_name).ok()?;
    // fs::read(&path).ok()

    let metadata_path = metadata_cache_path(app, pkg_name).ok()?;

    if let Ok(metadata_str) = fs::read_to_string(&metadata_path) {
        if let Ok(metadata) = serde_json::from_str::<CacheMetadata>(&metadata_str) {
            if metadata.is_expired() {
                let _ = fs::remove_file(&metadata_path);
                let _ = fs::remove_file(icon_cache_path(app, pkg_name).ok()?);

                return None;
            }
        }
    }

    let path = icon_cache_path(app, pkg_name).ok()?;
    fs::read(&path).ok()
}

pub fn write_cache_icon(
    app: &tauri::AppHandle,
    pkg_name: &str,
    png: &[u8],
    source: &str,
    ttl_seconds: u64,
) -> Result<(), String> {
    let path = icon_cache_path(app, pkg_name)?;
    fs::write(path, png).map_err(|e| e.to_string())?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let metadata = CacheMetadata {
        source: source.to_string(),
        cached_at: now,
        ttl_seconds,
    };

    let metadata_json = serde_json::to_string(&metadata).map_err(|e| e.to_string())?;

    let metadata_path = metadata_cache_path(app, pkg_name)?;
    fs::write(metadata_path, metadata_json).map_err(|e| e.to_string())
}
