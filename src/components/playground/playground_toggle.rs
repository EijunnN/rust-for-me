use leptos::prelude::*;
use crate::i18n::{use_i18n, Translations};
use super::playground_context::use_playground;

/// A button that toggles the Playground drawer open/closed.
#[component]
pub fn PlaygroundToggle() -> impl IntoView {
    let pg = use_playground();
    let i18n = use_i18n();

    let on_click = move |_| {
        pg.is_open.set(!pg.is_open.get());
    };

    view! {
        <button
            class="flex items-center px-3 py-1.5 rounded-lg text-sm font-medium bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-300 hover:bg-orange-200 dark:hover:bg-orange-900/50 transition-colors"
            on:click=on_click
            title="Ctrl+Shift+P"
        >
            // Terminal icon
            <svg class="w-4 h-4 mr-1.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
            {move || {
                let locale = i18n.locale.get();
                let tr = Translations::get(locale);
                tr.playground_tooltip.to_string()
            }}
        </button>
    }
}
