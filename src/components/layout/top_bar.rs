use leptos::prelude::*;
use leptos::ev;
use crate::i18n::use_i18n;
use crate::app::use_theme;
use crate::components::playground::playground_toggle::PlaygroundToggle;

#[component]
pub fn TopBar(
    on_toggle_sidebar: impl Fn(ev::MouseEvent) + 'static,
) -> impl IntoView {
    let i18n = use_i18n();
    let theme = use_theme();

    let toggle_locale = move |_| {
        let current = i18n.locale.get();
        i18n.locale.set(current.toggle());
    };

    let toggle_theme = move |_| {
        let current = theme.dark_mode.get();
        theme.dark_mode.set(!current);
    };

    view! {
        <header class="flex items-center justify-between h-16 px-6 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 shadow-sm">
            // Left side: hamburger + title
            <div class="flex items-center space-x-4">
                <button
                    class="p-2 rounded-lg text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                    on:click=on_toggle_sidebar
                >
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                    </svg>
                </button>
                <h1 class="text-lg font-semibold text-gray-800 dark:text-white">
                    "Rust for Everyone"
                </h1>
            </div>

            // Right side: playground + language toggle + theme toggle
            <div class="flex items-center space-x-3">
                // Playground toggle
                <PlaygroundToggle />

                // Language toggle
                <button
                    class="flex items-center px-3 py-1.5 rounded-lg text-sm font-medium bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
                    on:click=toggle_locale
                >
                    {move || {
                        let locale = i18n.locale.get();
                        format!("{}", locale.name())
                    }}
                </button>

                // Theme toggle
                <button
                    class="p-2 rounded-lg text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                    on:click=toggle_theme
                >
                    {move || if theme.dark_mode.get() {
                        // Sun icon for dark mode (click to go light)
                        view! {
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                            </svg>
                        }.into_any()
                    } else {
                        // Moon icon for light mode (click to go dark)
                        view! {
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                            </svg>
                        }.into_any()
                    }}
                </button>
            </div>
        </header>
    }
}
