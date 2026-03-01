use std::path::PathBuf;

/// Managed state holding the resolved path to the `content/` directory.
pub struct ContentDir(pub PathBuf);

/// Load a TOML content file from the `content/` directory.
/// The `path` argument should be a relative path such as
/// `theory/m01_introduction/01_what_is_rust.toml`.
#[tauri::command]
pub async fn load_content(
    path: String,
    content_dir: tauri::State<'_, ContentDir>,
) -> Result<String, String> {
    let full_path = content_dir.0.join(&path);

    let canonical = full_path
        .canonicalize()
        .map_err(|e| format!("File not found: {path} ({e})"))?;

    let content_canonical = content_dir
        .0
        .canonicalize()
        .map_err(|e| format!("Content directory not found: {e}"))?;

    if !canonical.starts_with(&content_canonical) {
        return Err("Access denied: path is outside the content directory".to_string());
    }

    tokio::fs::read_to_string(&canonical)
        .await
        .map_err(|e| format!("Failed to read {path}: {e}"))
}

/// List entries in a subdirectory of the `content/` folder.
/// Returns a JSON array of `{ "name": "...", "is_dir": bool }` entries.
#[tauri::command]
pub async fn list_content_dir(
    path: String,
    content_dir: tauri::State<'_, ContentDir>,
) -> Result<String, String> {
    let full_path = content_dir.0.join(&path);

    let canonical = full_path
        .canonicalize()
        .map_err(|e| format!("Directory not found: {path} ({e})"))?;

    let content_canonical = content_dir
        .0
        .canonicalize()
        .map_err(|e| format!("Content directory not found: {e}"))?;

    if !canonical.starts_with(&content_canonical) {
        return Err("Access denied: path is outside the content directory".to_string());
    }

    let mut entries = Vec::new();
    let mut read_dir = tokio::fs::read_dir(&canonical)
        .await
        .map_err(|e| format!("Failed to read directory {path}: {e}"))?;

    while let Some(entry) = read_dir
        .next_entry()
        .await
        .map_err(|e| format!("Failed to read entry: {e}"))?
    {
        let name = entry.file_name().to_string_lossy().to_string();
        let is_dir = entry
            .file_type()
            .await
            .map(|ft| ft.is_dir())
            .unwrap_or(false);
        entries.push(serde_json::json!({ "name": name, "is_dir": is_dir }));
    }

    serde_json::to_string(&entries).map_err(|e| format!("Failed to serialize: {e}"))
}
