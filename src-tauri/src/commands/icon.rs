use base64::{engine::general_purpose::STANDARD, Engine};
use tauri::command;

use crate::{
    services::fetch_logo_from_brandfetch,
    utils::{
        is_package_installed, read_cache_icon, read_installed_manifest, resolve_insalled_exe_path,
        write_cache_icon,
    },
};

fn png_to_data_url(png: &[u8]) -> String {
    format!("data:image/png;base64,{}", STANDARD.encode(png))
}

#[cfg(target_os = "windows")]
fn extract_exe_icon_png(exe_path: &std::path::Path) -> Result<Vec<u8>, String> {
    use image::ImageFormat;
    use std::io::Cursor;

    let icon = windows_icons::get_icon_by_path(exe_path.to_string_lossy().as_ref())
        .map_err(|e| format!("Failed to extract icon: {}", e))?;

    let mut png = Vec::new();

    icon.write_to(&mut Cursor::new(&mut png), ImageFormat::Png)
        .map_err(|e| format!("Failed to encode icon as PNG: {}", e))?;

    Ok(png)
}

#[cfg(not(target_os = "windows"))]
fn extract_exe_icon_png(_exe_path: &std::path::Path) -> Result<Vec<u8>, String> {
    Err("Icon extraction is only supported on Windows".into())
}

#[command]
pub async fn get_package_icon(
    app: tauri::AppHandle,
    pkg_name: String,
    homepage: Option<String>,
) -> Result<Option<String>, String> {
    if let Some(cached) = read_cache_icon(&app, &pkg_name) {
        return Ok(Some(png_to_data_url(&cached)));
    }

    if is_package_installed(&pkg_name)? {
        if let Ok(manifest) = read_installed_manifest(&pkg_name) {
            if let Some(bin) = manifest.bin.as_ref() {
                if let Ok(exe_path) = resolve_insalled_exe_path(&pkg_name, bin) {
                    if let Ok(png) = extract_exe_icon_png(&exe_path) {
                        let _ = write_cache_icon(&app, &pkg_name, &png, "native", u64::MAX);
                        return Ok(Some(png_to_data_url(&png)));
                    }
                }
            }
        }
    }

    if let Some(homepage_url) = homepage {
        if let Ok(url) = url::Url::parse(&homepage_url) {
            if let Some(domain) = url.domain() {
                let client = reqwest::Client::new();

                if let Ok(Some(logo_png)) = fetch_logo_from_brandfetch(domain, &client).await {
                    let ttl_30_days = 30 * 24 * 60 * 60;
                    let _ = write_cache_icon(&app, &pkg_name, &logo_png, "brandfetch", ttl_30_days);

                    return Ok(Some(png_to_data_url(&logo_png)));
                }
            }
        }
    }

    Ok(None)

    // if !is_package_installed(&pkg_name)? {
    //     return Ok(None);
    // }

    // let manifest = read_installed_manifest(&pkg_name).ok();
    // let Some(manifest) = manifest else {
    //     return Ok(None);
    // };
    // let Some(bin) = manifest.bin.as_ref() else {
    //     return Ok(None);
    // };

    // let exe_path = match resolve_insalled_exe_path(&pkg_name, bin) {
    //     Ok(path) => path,
    //     Err(_) => return Ok(None),
    // };

    // let png = extract_exe_icon_png(&exe_path).ok();
    // let Some(png) = png else {
    //     return Ok(None);
    // };

    // let _ = write_cache_icon(&app, &pkg_name, &png);

    // Ok(Some(png_to_data_url(&png)))
}
