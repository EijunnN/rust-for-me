use leptos::prelude::*;

use crate::i18n::{use_i18n, Translations};

/// Displays a colored badge for exercise/project difficulty level.
#[component]
pub fn DifficultyBadge(
    /// Difficulty level string: "beginner", "intermediate", or "advanced".
    #[prop(into)]
    difficulty: String,
) -> impl IntoView {
    let i18n = use_i18n();

    let diff_class = difficulty.clone();
    let diff_text = difficulty;

    view! {
        <span class=move || {
            let base = "inline-block px-2.5 py-0.5 rounded-full text-xs font-medium";
            match diff_class.as_str() {
                "beginner" => format!("{base} bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"),
                "intermediate" => format!("{base} bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200"),
                "advanced" => format!("{base} bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200"),
                _ => format!("{base} bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200"),
            }
        }>
            {move || {
                let tr = Translations::get(i18n.locale.get());
                match diff_text.as_str() {
                    "beginner" => tr.common_beginner.to_string(),
                    "intermediate" => tr.common_intermediate.to_string(),
                    "advanced" => tr.common_advanced.to_string(),
                    other => other.to_string(),
                }
            }}
        </span>
    }
}
