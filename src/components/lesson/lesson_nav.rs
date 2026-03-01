use leptos::prelude::*;
use leptos_router::components::A;
use crate::i18n::{use_i18n, Translations};

/// Previous/Next navigation for lessons.
#[component]
pub fn LessonNav(
    #[prop(optional, into)] prev_url: Option<String>,
    #[prop(optional, into)] next_url: Option<String>,
) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <nav class="flex items-center justify-between mt-8 pt-6 border-t border-gray-200 dark:border-gray-700">
            // Previous button
            <div>
                {match prev_url {
                    Some(url) => {
                        view! {
                            <A
                                href=url
                                attr:class="flex items-center px-4 py-2 rounded-lg text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
                            >
                                <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                                </svg>
                                {move || {
                                    let locale = i18n.locale.get();
                                    let tr = Translations::get(locale);
                                    tr.common_previous.to_string()
                                }}
                            </A>
                        }.into_any()
                    }
                    None => {
                        view! {
                            <span class="flex items-center px-4 py-2 rounded-lg text-gray-400 dark:text-gray-600 bg-gray-100 dark:bg-gray-800 cursor-not-allowed opacity-50">
                                <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                                </svg>
                                {move || {
                                    let locale = i18n.locale.get();
                                    let tr = Translations::get(locale);
                                    tr.common_previous.to_string()
                                }}
                            </span>
                        }.into_any()
                    }
                }}
            </div>

            // Next button
            <div>
                {match next_url {
                    Some(url) => {
                        view! {
                            <A
                                href=url
                                attr:class="flex items-center px-4 py-2 rounded-lg text-white bg-orange-600 hover:bg-orange-700 transition-colors shadow-md"
                            >
                                {move || {
                                    let locale = i18n.locale.get();
                                    let tr = Translations::get(locale);
                                    tr.common_next.to_string()
                                }}
                                <svg class="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                                </svg>
                            </A>
                        }.into_any()
                    }
                    None => {
                        view! {
                            <span class="flex items-center px-4 py-2 rounded-lg text-gray-400 bg-gray-100 dark:bg-gray-800 cursor-not-allowed opacity-50">
                                {move || {
                                    let locale = i18n.locale.get();
                                    let tr = Translations::get(locale);
                                    tr.common_next.to_string()
                                }}
                                <svg class="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                                </svg>
                            </span>
                        }.into_any()
                    }
                }}
            </div>
        </nav>
    }
}
