use leptos::prelude::*;

/// A simple textarea-based code editor.
/// In Phase 2 this will be replaced with a CodeMirror integration.
#[component]
pub fn CodeEditor(
    code: RwSignal<String>,
    #[prop(optional)] readonly: bool,
) -> impl IntoView {
    view! {
        <div class="relative rounded-lg overflow-hidden border border-gray-700">
            <div class="absolute top-2 right-2 text-xs text-gray-500 font-mono z-10 bg-gray-900 px-2 py-0.5 rounded">
                "Rust"
            </div>
            <textarea
                class="w-full h-64 p-4 bg-gray-900 text-green-400 font-mono text-sm resize-y focus:outline-none focus:ring-2 focus:ring-orange-500 leading-relaxed"
                prop:value=move || code.get()
                readonly=readonly
                spellcheck="false"
                autocomplete="off"
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    code.set(val);
                }
            />
        </div>
    }
}
