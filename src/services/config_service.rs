use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI_INTERNALS__"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

/// Retrieve a configuration setting by key.
/// Returns `None` if the key does not exist.
pub async fn get_setting(key: &str) -> Result<Option<String>, String> {
    let args = js_sys::Object::new();
    js_sys::Reflect::set(&args, &"key".into(), &key.into()).map_err(|e| format!("{:?}", e))?;

    let result = invoke("get_setting", args.into())
        .await
        .map_err(|e| format!("{:?}", e))?;

    if result.is_null() || result.is_undefined() {
        return Ok(None);
    }

    Ok(result.as_string())
}

/// Save a configuration setting with the given key and value.
pub async fn save_setting(key: &str, value: &str) -> Result<(), String> {
    let args = js_sys::Object::new();
    js_sys::Reflect::set(&args, &"key".into(), &key.into()).map_err(|e| format!("{:?}", e))?;
    js_sys::Reflect::set(&args, &"value".into(), &value.into())
        .map_err(|e| format!("{:?}", e))?;

    invoke("save_setting", args.into())
        .await
        .map_err(|e| format!("{:?}", e))?;

    Ok(())
}
