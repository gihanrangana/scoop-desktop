use std::path::PathBuf;

use serde_json::Value;

use crate::utils::get_apps_path;

pub fn resolve_bin_relative_path(bin: &Value) -> Option<String> {
    match bin {
        Value::String(s) => Some(s.replace('/', "\\")),
        Value::Array(arr) => arr.first().and_then(|i| match i {
            Value::String(s) => Some(s.replace('/', "\\")),
            Value::Array(pair) => pair.first().and_then(|v| v.as_str()).map(str::to_string),
            _ => None,
        }),
        Value::Object(map) => map
            .values()
            .next()
            .and_then(|v| v.as_str())
            .map(str::to_string),
        _ => None,
    }
}

pub fn resolve_insalled_exe_path(pkg_name: &str, bin: &Value) -> Result<PathBuf, String> {
    let apps_path = get_apps_path()?;
    let current = apps_path.join(pkg_name).join("current");

    let relative = resolve_bin_relative_path(bin)
        .ok_or_else(|| format!("Could not resolve bin for {}", pkg_name))?;

    let exe_path = current.join(&relative);

    if exe_path.exists() {
        return Ok(exe_path);
    }

    let fallback = current.join(format!("{}.exe", pkg_name));
    if fallback.exists() {
        return Ok(fallback);
    }

    Err(format!("Executable not found for {}", pkg_name))
}
