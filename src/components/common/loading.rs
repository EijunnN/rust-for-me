use leptos::prelude::*;

use crate::i18n::use_i18n;
use crate::i18n::locale::Locale;

/// A simple loading spinner component.
#[component]
pub fn Loading() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="flex items-center justify-center p-8">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-orange-500"></div>
            <span class="ml-3 text-gray-600 dark:text-gray-300">
                {move || match i18n.locale.get() {
                    Locale::Es => "Cargando...".to_string(),
                    Locale::En => "Loading...".to_string(),
                }}
            </span>
        </div>
    }
}
