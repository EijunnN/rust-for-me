use leptos::prelude::*;
use crate::i18n::{use_i18n, Translations};

/// Panel displayed when code compiles and runs successfully.
/// Shows encouraging feedback and, in exercise mode, whether the output matches expected.
#[component]
pub fn SuccessPanel(
    /// The actual output from running the code
    #[prop(into)]
    output: String,
    /// Optional expected output for exercise mode
    #[prop(optional, into)]
    expected_output: Option<String>,
) -> impl IntoView {
    let i18n = use_i18n();
    let output_text = output.clone();
    let expected = expected_output.clone();

    view! {
        {move || {
            let locale = i18n.locale.get();
            let tr = Translations::get(locale);
            let output_text = output_text.clone();
            let expected = expected.clone();

            match expected {
                Some(ref exp) => {
                    let trimmed_output = output_text.trim();
                    let trimmed_expected = exp.trim();
                    let passed = trimmed_output == trimmed_expected;

                    if passed {
                        view! {
                            <div class="rounded-lg border border-green-700/50 bg-green-950/30 p-4">
                                <div class="flex items-center mb-2">
                                    <span class="text-green-400 text-lg mr-2">{"\u{2713}"}</span>
                                    <h3 class="text-green-400 font-bold text-sm">{tr.success_title}</h3>
                                </div>
                                <p class="text-green-300 text-sm">{tr.success_exercise_passed}</p>
                                <div class="mt-3 p-2 bg-green-900/20 rounded font-mono text-xs text-green-300">
                                    <pre class="whitespace-pre-wrap">{output_text.clone()}</pre>
                                </div>
                                <p class="mt-2 text-xs text-green-500 italic">{tr.success_keep_going}</p>
                            </div>
                        }.into_any()
                    } else {
                        let exp_clone = exp.clone();
                        view! {
                            <div class="rounded-lg border border-yellow-700/50 bg-yellow-950/30 p-4">
                                <div class="flex items-center mb-2">
                                    <span class="text-yellow-400 text-lg mr-2">{"\u{2717}"}</span>
                                    <h3 class="text-yellow-400 font-bold text-sm">{tr.success_exercise_failed}</h3>
                                </div>
                                <div class="mt-2 space-y-2">
                                    <div class="p-2 bg-gray-900/50 rounded">
                                        <span class="text-xs text-gray-400 block mb-1">{tr.success_expected}{":"}</span>
                                        <pre class="font-mono text-xs text-green-400 whitespace-pre-wrap">{exp_clone}</pre>
                                    </div>
                                    <div class="p-2 bg-gray-900/50 rounded">
                                        <span class="text-xs text-gray-400 block mb-1">{tr.success_got}{":"}</span>
                                        <pre class="font-mono text-xs text-red-400 whitespace-pre-wrap">{output_text.clone()}</pre>
                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    }
                }
                None => {
                    view! {
                        <div class="rounded-lg border border-green-700/50 bg-green-950/30 p-4">
                            <div class="flex items-center mb-2">
                                <span class="text-green-400 text-lg mr-2">{"\u{2713}"}</span>
                                <h3 class="text-green-400 font-bold text-sm">{tr.success_title}</h3>
                            </div>
                            <p class="text-green-300 text-sm mb-2">{tr.success_compiled_ok}</p>
                            <div class="p-2 bg-gray-900/50 rounded font-mono text-xs text-green-300">
                                <pre class="whitespace-pre-wrap">{output_text.clone()}</pre>
                            </div>
                        </div>
                    }.into_any()
                }
            }
        }}
    }
}
