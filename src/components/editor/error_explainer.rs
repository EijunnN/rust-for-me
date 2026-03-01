use leptos::prelude::*;
use crate::i18n::{use_i18n, Translations};

/// Returns the beginner-friendly explanation for a known Rust error code.
/// Each entry contains (what, why, fix) translation keys looked up from the current locale.
fn get_error_explanation(
    error_code: &str,
    tr: &'static crate::i18n::translations::TranslationSet,
) -> Option<(&'static str, &'static str, &'static str)> {
    match error_code {
        "E0382" => Some((tr.error_e0382_what, tr.error_e0382_why, tr.error_e0382_fix)),
        "E0502" => Some((tr.error_e0502_what, tr.error_e0502_why, tr.error_e0502_fix)),
        "E0308" => Some((tr.error_e0308_what, tr.error_e0308_why, tr.error_e0308_fix)),
        "E0425" => Some((tr.error_e0425_what, tr.error_e0425_why, tr.error_e0425_fix)),
        "E0384" => Some((tr.error_e0384_what, tr.error_e0384_why, tr.error_e0384_fix)),
        "E0106" => Some((tr.error_e0106_what, tr.error_e0106_why, tr.error_e0106_fix)),
        _ => None,
    }
}

/// A collapsible panel that shows a beginner-friendly explanation for a Rust compiler error code.
/// When the user clicks the error code, the explanation expands to show:
/// - What the error means
/// - Why Rust prevents it
/// - How to fix it
#[component]
pub fn ErrorExplainer(
    /// The Rust compiler error code (e.g., "E0382")
    #[prop(into)]
    error_code: String,
) -> impl IntoView {
    let i18n = use_i18n();
    let (expanded, set_expanded) = signal(false);
    let code = error_code.clone();

    view! {
        {move || {
            let locale = i18n.locale.get();
            let tr = Translations::get(locale);
            let code = code.clone();

            if let Some((what, why, fix)) = get_error_explanation(&code, tr) {
                let toggle_label = if expanded.get() {
                    tr.error_hide_explanation
                } else {
                    tr.error_show_explanation
                };

                view! {
                    <div class="mt-2 rounded-lg border border-amber-700/50 bg-amber-950/30 overflow-hidden">
                        <button
                            class="w-full flex items-center justify-between px-3 py-2 text-xs text-amber-400 hover:bg-amber-900/30 transition-colors"
                            on:click=move |_| set_expanded.set(!expanded.get())
                        >
                            <span class="flex items-center">
                                <span class="mr-2 font-bold text-amber-300">{format!("[{}]", code)}</span>
                                <span>{toggle_label}</span>
                            </span>
                            <span class="text-amber-500">
                                {move || if expanded.get() { "\u{25B2}" } else { "\u{25BC}" }}
                            </span>
                        </button>

                        {move || if expanded.get() {
                            view! {
                                <div class="px-4 pb-3 space-y-3 border-t border-amber-700/30">
                                    // What this error means
                                    <div class="pt-3">
                                        <h4 class="text-xs font-bold text-blue-400 uppercase tracking-wide mb-1">
                                            {tr.error_what}
                                        </h4>
                                        <p class="text-sm text-gray-300 leading-relaxed">
                                            {what}
                                        </p>
                                    </div>

                                    // Why Rust prevents this
                                    <div>
                                        <h4 class="text-xs font-bold text-purple-400 uppercase tracking-wide mb-1">
                                            {tr.error_why}
                                        </h4>
                                        <p class="text-sm text-gray-300 leading-relaxed">
                                            {why}
                                        </p>
                                    </div>

                                    // How to fix it
                                    <div>
                                        <h4 class="text-xs font-bold text-green-400 uppercase tracking-wide mb-1">
                                            {tr.error_fix}
                                        </h4>
                                        <p class="text-sm text-gray-300 leading-relaxed">
                                            {fix}
                                        </p>
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }}
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }
        }}
    }
}
