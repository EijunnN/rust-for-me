use leptos::prelude::*;
use crate::components::editor::code_editor::CodeEditor;
use crate::components::editor::run_button::RunButton;
use crate::components::editor::output_panel::OutputPanel;

/// A code block that can optionally be runnable.
/// If runnable, shows an interactive editor with run button and output.
/// If not runnable, shows a read-only code block with a copy button.
#[component]
pub fn CodeExample(
    #[prop(into)] code: String,
    #[prop(optional)] runnable: bool,
) -> impl IntoView {
    if runnable {
        let code_signal = RwSignal::new(code);
        let (output, set_output) = signal(String::new());
        let (is_error, set_is_error) = signal(false);
        let (is_loading, set_is_loading) = signal(false);

        let on_run = move |_| {
            let current_code = code_signal.get();
            set_is_loading.set(true);
            set_output.set(String::new());
            set_is_error.set(false);

            leptos::task::spawn_local(async move {
                match crate::services::compiler_service::compile_and_run(&current_code).await {
                    Ok(result) => {
                        if result.success {
                            set_output.set(result.stdout);
                            set_is_error.set(false);
                        } else {
                            set_output.set(result.stderr);
                            set_is_error.set(true);
                        }
                    }
                    Err(e) => {
                        set_output.set(format!("Error: {e}"));
                        set_is_error.set(true);
                    }
                }
                set_is_loading.set(false);
            });
        };

        view! {
            <div class="my-6 space-y-3">
                <CodeEditor code=code_signal readonly=false />
                <div class="flex justify-end">
                    <RunButton on_click=on_run is_loading=is_loading />
                </div>
                <OutputPanel output=output is_error=is_error is_loading=is_loading />
            </div>
        }.into_any()
    } else {
        let code_for_copy = code.clone();
        let (copied, set_copied) = signal(false);

        let on_copy = move |_| {
            let code_text = code_for_copy.clone();
            leptos::task::spawn_local(async move {
                let _ = wasm_bindgen_futures::JsFuture::from(
                    web_sys::window()
                        .unwrap()
                        .navigator()
                        .clipboard()
                        .write_text(&code_text),
                )
                .await;
            });
            set_copied.set(true);
            // Reset after 2 seconds
            leptos::task::spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(2000).await;
                set_copied.set(false);
            });
        };

        view! {
            <div class="relative my-6 rounded-lg overflow-hidden border border-gray-700">
                <div class="flex items-center justify-between px-4 py-2 bg-gray-800 border-b border-gray-700">
                    <span class="text-xs text-gray-400 font-mono">"Rust"</span>
                    <button
                        class="text-xs text-gray-400 hover:text-white transition-colors px-2 py-1 rounded"
                        on:click=on_copy
                    >
                        {move || if copied.get() {
                            "\u{2713} Copied!"
                        } else {
                            "Copy"
                        }}
                    </button>
                </div>
                <pre class="p-4 bg-gray-900 text-green-400 font-mono text-sm overflow-x-auto leading-relaxed">
                    <code>{code}</code>
                </pre>
            </div>
        }.into_any()
    }
}
