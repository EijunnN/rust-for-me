use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "CMBridge"], js_name = "create")]
    fn cm_create(element: &web_sys::HtmlElement, initial_code: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "CMBridge"], js_name = "getValue")]
    fn cm_get_value(view: &JsValue) -> String;

    #[wasm_bindgen(js_namespace = ["window", "CMBridge"], js_name = "setValue")]
    fn cm_set_value(view: &JsValue, text: &str);

    #[wasm_bindgen(js_namespace = ["window", "CMBridge"], js_name = "destroy")]
    fn cm_destroy(view: &JsValue);
}

/// A Leptos component wrapping a CodeMirror 6 editor for Rust code.
///
/// The editor is created the first time `is_visible` becomes true so that
/// the container has real pixel dimensions for CodeMirror to measure.
#[component]
pub fn CodeMirrorEditor(
    code: RwSignal<String>,
    is_visible: RwSignal<bool>,
) -> impl IntoView {
    let container_ref = NodeRef::<leptos::html::Div>::new();
    let editor_view: StoredValue<Option<JsValue>> = StoredValue::new(None);
    let editor_ready = RwSignal::new(false);
    let creating = StoredValue::new(false);

    // Create the CodeMirror editor the first time the drawer opens.
    // The bundle is loaded synchronously so CMBridge is always available.
    Effect::new(move |_| {
        if !is_visible.get() {
            return;
        }
        if editor_ready.get_untracked() || creating.get_value() {
            return;
        }
        if let Some(el) = container_ref.get() {
            creating.set_value(true);
            let code_val = code.get_untracked();
            // Use requestAnimationFrame so the drawer has pixel dimensions
            leptos::task::spawn_local(async move {
                // One frame delay for layout to settle
                gloo_timers::future::TimeoutFuture::new(60).await;
                let html_el: &web_sys::HtmlElement = &el;
                let view = cm_create(html_el, &code_val);
                if !view.is_null() && !view.is_undefined() {
                    editor_view.set_value(Some(view));
                    editor_ready.set(true);
                }
                creating.set_value(false);
            });
        }
    });

    // Sync external code changes into CodeMirror.
    Effect::new(move |prev: Option<String>| {
        let current = code.get();
        if !editor_ready.get() {
            return current;
        }
        if let Some(prev_val) = prev {
            if prev_val != current {
                editor_view.with_value(|v| {
                    if let Some(view) = v {
                        let editor_text = cm_get_value(view);
                        if editor_text != current {
                            cm_set_value(view, &current);
                        }
                    }
                });
            }
        }
        current
    });

    // Sync CodeMirror → code signal on blur and keyup.
    let sync_to_signal = move || {
        if !editor_ready.get_untracked() {
            return;
        }
        editor_view.with_value(|v| {
            if let Some(view) = v {
                let text = cm_get_value(view);
                if text != code.get_untracked() {
                    code.set(text);
                }
            }
        });
    };

    on_cleanup(move || {
        editor_view.with_value(|v| {
            if let Some(view) = v {
                cm_destroy(view);
            }
        });
    });

    let sync_clone = sync_to_signal.clone();

    view! {
        <div
            node_ref=container_ref
            class="absolute inset-0"
            on:focusout=move |_| sync_clone()
            on:keyup=move |_| sync_to_signal()
        />
    }
}
