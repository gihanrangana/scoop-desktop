use crate::utils::{count_bucket_packages, get_bucket_names};
use serde::{Deserialize, Serialize};
use tauri::command;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bucket {
    pub name: String,
    pub package_count: usize,
}

#[command]
pub async fn get_buckets() -> Result<Vec<Bucket>, String> {
    let bucket_names = get_bucket_names()?;
    let mut buckets = Vec::new();

    for name in bucket_names {
        let package_count = count_bucket_packages(&name).unwrap_or(0);

        buckets.push(Bucket {
            name,
            package_count,
        });
    }

    Ok(buckets)
}
