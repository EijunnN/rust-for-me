use leptos::prelude::*;
use leptos::ev;
use crate::i18n::{use_i18n, Translations};

/// Run / compile button with loading state.
#[component]
pub fn RunButton(
    on_click: impl Fn(ev::MouseEvent) + 'static,
    #[prop(into)] is_loading: Signal<bool>,
) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <button
            class="flex items-center px-4 py-2 rounded-lg font-medium text-white bg-orange-600 hover:bg-orange-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors shadow-md"
            on:click=on_click
            disabled=move || is_loading.get()
        >
            {move || if is_loading.get() {
                view! {
                    <span class="flex items-center">
                        <span class="animate-spin inline-block w-4 h-4 border-2 border-white border-t-transparent rounded-full mr-2"></span>
                        "..."
                    </span>
                }.into_any()
            } else {
                let locale = i18n.locale.get();
                let tr = Translations::get(locale);
                view! {
                    <span class="flex items-center">
                        <span class="mr-2">{"\u{25B6}"}</span>
                        {tr.common_run.to_string()}
                    </span>
                }.into_any()
            }}
        </button>
    }
}
