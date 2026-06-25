use crate::utils::{get_bucket_names, get_bucket_path, read_manifest, ScoopManifest};
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::command;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub bucket: String,
    pub homepage: String,
    pub notes: Vec<String>,
    pub license: Option<String>,
}

fn read_bucket_packages(bucket_name: &str) -> Result<Vec<Package>, String> {
    let bucket_path = get_bucket_path(bucket_name)?;
    let mut packages = Vec::new();

    if let Ok(entries) = fs::read_dir(&bucket_path) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    if let Ok(manifest) = read_manifest(bucket_name, name) {
                        packages.push(Package {
                            name: name.to_string(),
                            version: manifest.version,
                            description: manifest.description.to_string(),
                            homepage: manifest.homepage.unwrap_or_default(),
                            notes: manifest.notes.unwrap_or_default(),
                            license: manifest.license,
                            bucket: bucket_name.to_string(),
                        });
                    }
                }
            }
        }
    }

    Ok(packages)
}

#[command]
pub async fn get_all_packages() -> Result<Vec<Package>, String> {
    let buckets = get_bucket_names()?;
    let mut all_packages = Vec::new();

    for bucket in buckets {
        let mut packages = read_bucket_packages(&bucket)?;
        all_packages.append(&mut packages);
    }

    Ok(all_packages)
}

#[command]
pub async fn get_packages_by_bucket(bucket_name: String) -> Result<Vec<Package>, String> {
    read_bucket_packages(&bucket_name)
}

#[command]
pub async fn search_packages(query: String) -> Result<Vec<Package>, String> {
    let all_packages = get_all_packages().await?;
    let query_lower = query.to_lowercase();

    let filtered: Vec<Package> = all_packages
        .into_iter()
        .filter(|pkg| {
            pkg.name.to_lowercase().contains(&query_lower)
                || pkg.description.to_lowercase().contains(&query_lower)
        })
        .collect();

    Ok(filtered)
}

#[command]
pub async fn get_package_info(name: String, bucket: String) -> Result<ScoopManifest, String> {
    let manifest = read_manifest(&bucket, &name)?;

    Ok(manifest)
}
