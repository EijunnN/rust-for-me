use leptos::prelude::*;

use crate::app::use_theme;
use crate::i18n::locale::Locale;
use crate::i18n::{use_i18n, Translations};

/// Application settings page.
/// Allows the user to change language, theme, configure the Rust toolchain path,
/// and view About information.
#[component]
pub fn Settings() -> impl IntoView {
    let i18n = use_i18n();
    let theme = use_theme();
    let (rust_path, set_rust_path) = signal(String::from("rustc"));

    // Load the saved rust path on mount
    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(Some(path)) = crate::services::config_service::get_setting("rust_path").await
            {
                set_rust_path.set(path);
            }
        });
    });

    let save_rust_path = move |_| {
        let path = rust_path.get();
        leptos::task::spawn_local(async move {
            let _ = crate::services::config_service::save_setting("rust_path", &path).await;
        });
    };

    view! {
        <div class="max-w-2xl mx-auto space-y-8">
            <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
                {move || {
                    let tr = Translations::get(i18n.locale.get());
                    tr.settings_title.to_string()
                }}
            </h1>

            // Language setting
            <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
                <h2 class="text-lg font-semibold text-gray-800 dark:text-gray-200 mb-4">
                    {move || {
                        let tr = Translations::get(i18n.locale.get());
                        tr.settings_language.to_string()
                    }}
                </h2>
                <div class="flex gap-3">
                    <button
                        class=move || {
                            let base = "px-4 py-2 rounded-lg font-medium text-sm transition-colors";
                            if i18n.locale.get() == Locale::Es {
                                format!("{base} bg-orange-500 text-white")
                            } else {
                                format!(
                                    "{base} bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600",
                                )
                            }
                        }
                        on:click=move |_| i18n.locale.set(Locale::Es)
                    >
                        {"\u{1F1EA}\u{1F1F8} Espa\u{00F1}ol"}
                    </button>
                    <button
                        class=move || {
                            let base = "px-4 py-2 rounded-lg font-medium text-sm transition-colors";
                            if i18n.locale.get() == Locale::En {
                                format!("{base} bg-orange-500 text-white")
                            } else {
                                format!(
                                    "{base} bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600",
                                )
                            }
                        }
                        on:click=move |_| i18n.locale.set(Locale::En)
                    >
                        {"\u{1F1EC}\u{1F1E7} English"}
                    </button>
                </div>
            </div>

            // Theme setting
            <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
                <h2 class="text-lg font-semibold text-gray-800 dark:text-gray-200 mb-4">
                    {move || {
                        let tr = Translations::get(i18n.locale.get());
                        tr.settings_theme.to_string()
                    }}
                </h2>
                <div class="flex gap-3">
                    <button
                        class=move || {
                            let base = "flex items-center px-4 py-2 rounded-lg font-medium text-sm transition-colors";
                            if !theme.dark_mode.get() {
                                format!("{base} bg-orange-500 text-white")
                            } else {
                                format!(
                                    "{base} bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600",
                                )
                            }
                        }
                        on:click=move |_| theme.dark_mode.set(false)
                    >
                        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                        </svg>
                        "Light"
                    </button>
                    <button
                        class=move || {
                            let base = "flex items-center px-4 py-2 rounded-lg font-medium text-sm transition-colors";
                            if theme.dark_mode.get() {
                                format!("{base} bg-orange-500 text-white")
                            } else {
                                format!(
                                    "{base} bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600",
                                )
                            }
                        }
                        on:click=move |_| theme.dark_mode.set(true)
                    >
                        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                        </svg>
                        "Dark"
                    </button>
                </div>
            </div>

            // Rust path setting
            <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
                <h2 class="text-lg font-semibold text-gray-800 dark:text-gray-200 mb-2">
                    {move || {
                        let tr = Translations::get(i18n.locale.get());
                        tr.settings_rust_path.to_string()
                    }}
                </h2>
                <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">
                    {move || match i18n.locale.get() {
                        Locale::Es => "Ruta al compilador de Rust en tu sistema.".to_string(),
                        Locale::En => "Path to the Rust compiler on your system.".to_string(),
                    }}
                </p>
                <div class="flex gap-3">
                    <input
                        type="text"
                        class="flex-1 px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-100 font-mono text-sm focus:ring-2 focus:ring-orange-500 focus:outline-none"
                        prop:value=move || rust_path.get()
                        on:input=move |ev| {
                            set_rust_path.set(event_target_value(&ev));
                        }
                    />
                    <button
                        class="px-4 py-2 rounded-lg bg-orange-500 hover:bg-orange-600 text-white font-medium text-sm transition-colors"
                        on:click=save_rust_path
                    >
                        {move || match i18n.locale.get() {
                            Locale::Es => "Guardar".to_string(),
                            Locale::En => "Save".to_string(),
                        }}
                    </button>
                </div>
            </div>

            // About section
            <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
                <h2 class="text-lg font-semibold text-gray-800 dark:text-gray-200 mb-4">
                    {move || match i18n.locale.get() {
                        Locale::Es => "Acerca de".to_string(),
                        Locale::En => "About".to_string(),
                    }}
                </h2>
                <div class="space-y-3">
                    <div class="flex items-center gap-3">
                        <span class="text-3xl">{"\u{1F980}"}</span>
                        <div>
                            <p class="font-bold text-gray-800 dark:text-gray-200">"Rust for Everyone"</p>
                            <p class="text-sm text-gray-500 dark:text-gray-400">"v0.1.0"</p>
                        </div>
                    </div>
                    <p class="text-sm text-gray-600 dark:text-gray-400">
                        {move || match i18n.locale.get() {
                            Locale::Es => "Una plataforma interactiva para aprender Rust paso a paso. Incluye teoria, ejercicios practicos y proyectos guiados.".to_string(),
                            Locale::En => "An interactive platform to learn Rust step by step. Includes theory, practical exercises and guided projects.".to_string(),
                        }}
                    </p>
                    <div class="pt-3 border-t border-gray-200 dark:border-gray-700">
                        <p class="text-xs text-gray-400 dark:text-gray-500">
                            {move || match i18n.locale.get() {
                                Locale::Es => "Construido con Leptos + Tauri + Tailwind CSS".to_string(),
                                Locale::En => "Built with Leptos + Tauri + Tailwind CSS".to_string(),
                            }}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}
