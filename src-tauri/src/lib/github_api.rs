use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub files: Vec<String>,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct GithubFile {
    name: String,
    path: String,
    #[serde(rename = "type")]
    file_type: String,
}

pub async fn get_available_buckets() -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();

    let response = client
        .get(
            "https://raw.githubusercontent.com/ScoopInstaller/Scoop/refs/heads/master/buckets.json",
        )
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let buckets = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON response: {}", e))?;

    Ok(buckets)
}

pub async fn fetch_json_files() -> Result<ApiResponse, String> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://api.github.com/repos/gihanrangana/scoop-Main/contents/bucket")
        .header("User-Agent", "Scoop Desktop App")
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let files: Vec<GithubFile> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON response: {}", e))?;

    let json_files: Vec<String> = files
        .into_iter()
        .filter(|file| file.name.ends_with(".json"))
        .map(|file| file.name)
        .collect();

    let count = json_files.len();
    Ok(ApiResponse {
        files: json_files,
        count,
    })
}
