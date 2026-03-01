use leptos::prelude::*;
use crate::i18n::use_i18n;
use crate::i18n::locale::Locale;

/// An option for the quiz with localized text and a correctness flag.
#[derive(Clone, Debug)]
pub struct QuizOptionData {
    pub es: String,
    pub en: String,
    pub correct: bool,
}

/// An inline quiz component that presents a question with multiple-choice options.
#[component]
pub fn InteractiveQuiz(
    #[prop(into)] question_es: String,
    #[prop(into)] question_en: String,
    options: Vec<QuizOptionData>,
) -> impl IntoView {
    let i18n = use_i18n();
    let (selected, set_selected) = signal(Option::<usize>::None);
    let (submitted, set_submitted) = signal(false);

    let options_clone = options.clone();

    let is_correct = move || {
        if let Some(idx) = selected.get() {
            options_clone.get(idx).map(|o| o.correct).unwrap_or(false)
        } else {
            false
        }
    };

    let question_es = question_es.clone();
    let question_en = question_en.clone();

    view! {
        <div class="my-6 p-6 bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 shadow-sm">
            // Question
            <div class="flex items-start mb-4">
                <span class="flex-shrink-0 w-8 h-8 flex items-center justify-center rounded-full bg-orange-100 dark:bg-orange-900 text-orange-600 dark:text-orange-400 font-bold text-sm mr-3">
                    "?"
                </span>
                <p class="text-lg font-medium text-gray-800 dark:text-gray-200">
                    {move || match i18n.locale.get() {
                        Locale::Es => question_es.clone(),
                        Locale::En => question_en.clone(),
                    }}
                </p>
            </div>

            // Options
            <div class="space-y-2 ml-11 mb-4">
                {options.into_iter().enumerate().map(|(idx, opt)| {
                    let opt_es = opt.es.clone();
                    let opt_en = opt.en.clone();
                    let correct = opt.correct;

                    view! {
                        <label
                            class=move || {
                                let base = "flex items-center p-3 rounded-lg border cursor-pointer transition-all";
                                let is_selected = selected.get() == Some(idx);
                                if submitted.get() {
                                    if correct {
                                        format!("{base} border-green-500 bg-green-50 dark:bg-green-900/30")
                                    } else if is_selected && !correct {
                                        format!("{base} border-red-500 bg-red-50 dark:bg-red-900/30")
                                    } else {
                                        format!("{base} border-gray-200 dark:border-gray-600 opacity-60")
                                    }
                                } else if is_selected {
                                    format!("{base} border-orange-500 bg-orange-50 dark:bg-orange-900/20")
                                } else {
                                    format!("{base} border-gray-200 dark:border-gray-600 hover:border-orange-300 hover:bg-gray-50 dark:hover:bg-gray-700")
                                }
                            }
                        >
                            <input
                                type="radio"
                                name="quiz"
                                class="form-radio text-orange-500 mr-3"
                                checked=move || selected.get() == Some(idx)
                                disabled=move || submitted.get()
                                on:change=move |_| {
                                    set_selected.set(Some(idx));
                                }
                            />
                            <span class="text-gray-700 dark:text-gray-300">
                                {move || match i18n.locale.get() {
                                    Locale::Es => opt_es.clone(),
                                    Locale::En => opt_en.clone(),
                                }}
                            </span>
                            {move || {
                                if submitted.get() && correct {
                                    view! { <span class="ml-auto text-green-500 font-bold">{"\u{2713}"}</span> }.into_any()
                                } else if submitted.get() && selected.get() == Some(idx) && !correct {
                                    view! { <span class="ml-auto text-red-500 font-bold">{"\u{2717}"}</span> }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }
                            }}
                        </label>
                    }
                }).collect::<Vec<_>>()}
            </div>

            // Submit button and feedback
            <div class="ml-11">
                {move || if !submitted.get() {
                    view! {
                        <button
                            class="px-4 py-2 rounded-lg font-medium text-white bg-orange-600 hover:bg-orange-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                            disabled=move || selected.get().is_none()
                            on:click=move |_| set_submitted.set(true)
                        >
                            "Submit"
                        </button>
                    }.into_any()
                } else {
                    let correct = is_correct();
                    if correct {
                        view! {
                            <div class="flex items-center p-3 rounded-lg bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400">
                                <span class="font-bold mr-2">{"\u{2713}"}</span>
                                <span class="font-medium">
                                    {move || match i18n.locale.get() {
                                        Locale::Es => "\u{00A1}Correcto! Bien hecho.".to_string(),
                                        Locale::En => "Correct! Well done.".to_string(),
                                    }}
                                </span>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="flex items-center p-3 rounded-lg bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-400">
                                <span class="font-bold mr-2">{"\u{2717}"}</span>
                                <span class="font-medium">
                                    {move || match i18n.locale.get() {
                                        Locale::Es => "Incorrecto. Intenta de nuevo revisando el material.".to_string(),
                                        Locale::En => "Incorrect. Try again after reviewing the material.".to_string(),
                                    }}
                                </span>
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
