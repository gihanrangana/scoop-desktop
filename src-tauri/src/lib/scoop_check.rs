use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoopStatus {
    pub installed: bool,
    pub path: Option<String>,
    pub version: Option<String>,
    pub update_available: bool,
}

pub async fn check_scoop_installed(app_handle: AppHandle) -> ScoopStatus {
    // let output = Command::new("powershell")
    //     .args(["-Command", "Get-Command scoop -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source"])
    //     .output();

    let is_not_installed = check_is_installed(app_handle.clone()).await;

    if is_not_installed {
        return ScoopStatus {
            installed: false,
            path: None,
            version: None,
            update_available: false,
        };
    } else {
        let path = get_scoop_path(app_handle.clone()).await;
        let version = get_scoop_version(app_handle.clone()).await;
        let update_available = check_for_update(app_handle.clone()).await;

        return ScoopStatus {
            installed: true,
            path,
            version,
            update_available,
        };
    }
}

async fn check_is_installed(app_handle: AppHandle) -> bool {
    let shell = app_handle.shell();

    let is_installed = match shell
        .command("powershell")
        .args([
            "-Command",
            "Get-Command scoop -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source"
        ])
        .output()
        .await {
            Ok(output) => {
                if output.status.success() {
                    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !path.is_empty() {
                        Some(path)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Err(_) => None,
        };

    return is_installed.is_none();
}

async fn get_scoop_path(app_handle: AppHandle) -> Option<String> {
    let shell = app_handle.shell();

    match shell
        .command("powershell")
        .args([
            "-Command",
            "Get-Command scoop -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source"
        ])
        .output()
        .await {
            Ok(output) => {
                if output.status.success() {
                    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !path.is_empty() {
                        Some(path)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Err(_) => None,
        }
}

async fn get_scoop_version(app_handle: AppHandle) -> Option<String> {
    let shell = app_handle.shell();

    match shell
        .command("powershell")
        .args(["-Command", "scoop --version"])
        .output()
        .await
    {
        Ok(output) => {
            if output.status.success() {
                let version_output = String::from_utf8_lossy(&output.stdout).trim().to_string();

                // Based on your example output: "Current Scoop version: 859d1db5 (HEAD -> master, tag: v0.5.2..."
                let lines: Vec<&str> = version_output.lines().collect();
                if lines.len() > 0 {
                    if let Some(version_line) = lines
                        .iter()
                        .find(|line| line.contains("Current Scoop version:"))
                    {
                        // Extract the version tag (v0.5.2)
                        if let Some(tag_start) = version_line.find("tag: v") {
                            let tag_part = &version_line[tag_start + 5..]; // Skip "tag: v"
                            if let Some(tag_end) = tag_part.find(")") {
                                return Some(tag_part[..tag_end].to_string());
                            } else if let Some(tag_end) = tag_part.find(",") {
                                return Some(tag_part[..tag_end].to_string());
                            } else {
                                // Fallback to returning the whole version line
                                return Some(version_line.trim().to_string());
                            }
                        } else {
                            // If no tag found, return the commit hash as version
                            return Some(version_line.split(" ").nth(3).unwrap_or("").to_string());
                        }
                    }
                }
                // Fallback to returning the raw output if we couldn't parse it
                Some(version_output)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

async fn check_for_update(app_handle: AppHandle) -> bool {
    let shell = app_handle.shell();

    match shell
        .command("powershell")
        .args(["-Command", "scoop status"])
        .output()
        .await
    {
        Ok(output) => {
            if output.status.success() {
                let status_output = String::from_utf8_lossy(&output.stdout).to_string();

                // Check if there are updates available
                // If updates are available, the output typically contains "Updates are available"
                // or lists apps that need updating, including possibly "scoop" itself
                !status_output.contains("Everything is up to date")
                    && (status_output.contains("Updates are available")
                        || status_output.contains("scoop: "))
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

pub async fn install_scoop(app_handle: AppHandle) -> Result<String, String> {
    let shell = app_handle.shell();

    let output = shell
        .command("powershell")
        .args([
            "-Command",
            "Set-ExecutionPolicy RemoteSigned -Scope CurrentUser; Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression"
        ])
        .output()
        .await
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        Ok("Scoop was installed successfully.".to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to install Scoop: {}", error))
    }
}
