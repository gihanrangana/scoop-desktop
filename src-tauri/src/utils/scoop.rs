use serde::{Deserialize, Deserializer, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScoopManifest {
    #[serde(skip, default)]
    pub name: String,
    #[serde(skip, default)]
    pub bucket: String,
    pub version: String,
    #[serde(deserialize_with = "deserialize_description")]
    pub description: String,
    pub homepage: Option<String>,
    pub license: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub notes: Option<Vec<String>>,
    pub architecture: Option<serde_json::Value>,
    pub depends: Option<Vec<String>>,
    pub bin: Option<serde_json::Value>,
}

pub fn get_scoop_path() -> Result<PathBuf, String> {
    if let Ok(scoop_path) = std::env::var("SCOOP") {
        return Ok(PathBuf::from(scoop_path));
    }

    let home =
        std::env::var("USERPROFILE").map_err(|_| "Could not find user profile".to_string())?;

    Ok(PathBuf::from(home).join("scoop"))
}

pub fn get_bucket_names() -> Result<Vec<String>, String> {
    let scoop_path = get_scoop_path()?;
    let buckets_path = scoop_path.join("buckets");

    let mut buckets = Vec::new();

    if !buckets_path.exists() {
        return Ok(buckets);
    }

    if let Ok(entries) = fs::read_dir(&buckets_path) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    buckets.push(name.to_string());
                }
            }
        }
    }

    Ok(buckets)
}

pub fn get_bucket_path(bucket_name: &str) -> Result<PathBuf, String> {
    let scoop_path = get_scoop_path()?;
    let bucket_path = scoop_path.join("buckets").join(bucket_name).join("bucket");

    if !bucket_path.exists() {
        return Err(format!("Bucket '{}' not found", bucket_name));
    }

    Ok(bucket_path)
}

pub fn get_apps_path() -> Result<PathBuf, String> {
    let scoop_path = get_scoop_path()?;
    Ok(scoop_path.join("apps"))
}

pub fn read_manifest(bucket_name: &str, package_name: &str) -> Result<ScoopManifest, String> {
    let bucket_path = get_bucket_path(bucket_name)?;
    let manifest_path = bucket_path.join(format!("{}.json", package_name));

    let content = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read manifest: {}", e))?;

    let mut manifest: ScoopManifest =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse manifest: {}", e))?;

    manifest.name = package_name.to_string();
    manifest.bucket = bucket_name.to_string();

    Ok(manifest)
}

pub fn count_bucket_packages(bucket_name: &str) -> Result<usize, String> {
    let bucket_path = get_bucket_path(bucket_name)?;

    let count = fs::read_dir(&bucket_path)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
                .count()
        })
        .unwrap_or(0);

    Ok(count)
}

pub fn is_package_installed(package_name: &str) -> Result<bool, String> {
    let apps_path = get_apps_path()?;
    let package_path = apps_path.join(package_name);

    Ok(package_path.exists())
}

pub fn get_installed_version(package_name: &str) -> Result<String, String> {
    Ok(read_installed_manifest(package_name)?.version)
}

pub fn get_install_timestamp(pkg_name: &str) -> Result<i64, String> {
    let apps_path = get_apps_path()?;
    let install_path = apps_path
        .join(pkg_name)
        .join("current")
        .join("install.json");

    let metadata =
        fs::metadata(&install_path).map_err(|e| format!("Failed to read metadata: {}", e))?;
    let last_modified = metadata
        .modified()
        .map_err(|e| format!("Failed to get modified time: {}", e))?;

    last_modified
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .map_err(|e| format!("Failed to compute timestamp: {}", e))
}

pub fn read_installed_manifest(package_name: &str) -> Result<ScoopManifest, String> {
    let apps_path = get_apps_path()?;
    let manifest_path = apps_path
        .join(package_name)
        .join("current")
        .join("manifest.json");

    let content = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read installed manifest: {}", e))?;

    let mut manifest: ScoopManifest =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse manifest: {}", e))?;

    manifest.name = package_name.to_string();

    Ok(manifest)
}

pub fn deserialize_string_or_vec<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;

    Ok(match value {
        None => None,
        Some(serde_json::Value::String(s)) => Some(vec![s]),
        Some(serde_json::Value::Array(arr)) => Some(
            arr.into_iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect(),
        ),
        _ => None,
    })
}

fn deserialize_description<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::String(s) => Ok(s),
        serde_json::Value::Array(arr) => Ok(arr
            .into_iter()
            .filter_map(|v| v.as_str().map(str::to_string))
            .collect::<Vec<_>>()
            .join(" ")),
        _ => Ok(String::new()),
    }
}
