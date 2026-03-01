use crate::models::progress::UserProgress;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI_INTERNALS__"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

/// Retrieve progress for a specific item by its ID and category.
pub async fn get_progress(id: &str, category: &str) -> Result<Option<UserProgress>, String> {
    let args = js_sys::Object::new();
    js_sys::Reflect::set(&args, &"id".into(), &id.into()).map_err(|e| format!("{:?}", e))?;
    js_sys::Reflect::set(&args, &"category".into(), &category.into())
        .map_err(|e| format!("{:?}", e))?;

    let result = invoke("get_progress", args.into())
        .await
        .map_err(|e| format!("{:?}", e))?;

    if result.is_null() || result.is_undefined() {
        return Ok(None);
    }

    let json_str = js_sys::JSON::stringify(&result).map_err(|e| format!("{:?}", e))?;
    let json_string: String = json_str.into();
    let progress: UserProgress =
        serde_json::from_str(&json_string).map_err(|e| e.to_string())?;
    Ok(Some(progress))
}

/// Save progress for a specific item.
/// The backend expects separate `id`, `category`, `status`, `score` parameters.
pub async fn save_progress(progress: &UserProgress) -> Result<(), String> {
    let status_str = match progress.status {
        crate::models::progress::ProgressStatus::Locked => "locked",
        crate::models::progress::ProgressStatus::Available => "available",
        crate::models::progress::ProgressStatus::InProgress => "in_progress",
        crate::models::progress::ProgressStatus::Completed => "completed",
    };

    let args = js_sys::Object::new();
    js_sys::Reflect::set(&args, &"id".into(), &progress.id.as_str().into())
        .map_err(|e| format!("{:?}", e))?;
    js_sys::Reflect::set(&args, &"category".into(), &progress.category.as_str().into())
        .map_err(|e| format!("{:?}", e))?;
    js_sys::Reflect::set(&args, &"status".into(), &status_str.into())
        .map_err(|e| format!("{:?}", e))?;
    js_sys::Reflect::set(&args, &"score".into(), &JsValue::from(progress.score))
        .map_err(|e| format!("{:?}", e))?;

    invoke("save_progress", args.into())
        .await
        .map_err(|e| format!("{:?}", e))?;

    Ok(())
}

/// Retrieve all progress entries, optionally filtered by category.
pub async fn get_all_progress(category: Option<&str>) -> Result<Vec<UserProgress>, String> {
    let args = js_sys::Object::new();
    if let Some(cat) = category {
        js_sys::Reflect::set(&args, &"category".into(), &cat.into())
            .map_err(|e| format!("{:?}", e))?;
    }

    let result = invoke("get_all_progress", args.into())
        .await
        .map_err(|e| format!("{:?}", e))?;

    let json_str = js_sys::JSON::stringify(&result).map_err(|e| format!("{:?}", e))?;
    let json_string: String = json_str.into();
    serde_json::from_str(&json_string).map_err(|e| e.to_string())
}
