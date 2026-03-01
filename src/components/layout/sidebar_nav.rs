use leptos::prelude::*;
use leptos_router::components::A;
use crate::i18n::{use_i18n, Translations};

#[component]
pub fn SidebarNav(
    is_open: ReadSignal<bool>,
    toggle: WriteSignal<bool>,
) -> impl IntoView {
    let i18n = use_i18n();

    let nav_items = move || {
        let locale = i18n.locale.get();
        let tr = Translations::get(locale);
        vec![
            ("/", tr.nav_dashboard, "dashboard"),
            ("/theory", tr.nav_theory, "theory"),
            ("/practice", tr.nav_practice, "practice"),
            ("/projects", tr.nav_projects, "projects"),
            ("/settings", tr.nav_settings, "settings"),
        ]
    };

    view! {
        <aside class=move || {
            let base = "flex flex-col bg-gray-800 dark:bg-gray-950 text-white transition-all duration-300 ease-in-out border-r border-gray-700";
            if is_open.get() {
                format!("{base} w-64")
            } else {
                format!("{base} w-16")
            }
        }>
            // Logo / Title area
            <div class="flex items-center h-16 px-4 border-b border-gray-700">
                <button
                    class="text-orange-500 font-bold text-xl mr-2 hover:text-orange-400 transition-colors"
                    on:click=move |_| toggle.set(!is_open.get())
                >
                    {move || if is_open.get() {
                        view! { <span class="text-2xl">{"\u{1F980}"}</span> }.into_any()
                    } else {
                        view! { <span class="text-2xl">{"\u{1F980}"}</span> }.into_any()
                    }}
                </button>
                {move || if is_open.get() {
                    view! { <span class="font-bold text-sm tracking-wide">"Rust for Everyone"</span> }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>

            // Navigation links
            <nav class="flex-1 py-4">
                <ul class="space-y-1 px-2">
                    {move || {
                        nav_items().into_iter().map(|(href, label, icon_type)| {
                            let open = is_open.get();
                            view! {
                                <li>
                                    <A
                                        href=href
                                        attr:class="flex items-center px-3 py-2.5 rounded-lg text-gray-300 hover:bg-gray-700 hover:text-orange-400 transition-colors group"
                                    >
                                        <span class="flex-shrink-0 w-5 h-5" inner_html=nav_icon(icon_type) />
                                        {if open {
                                            view! { <span class="ml-3 text-sm font-medium">{label.to_string()}</span> }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                    </A>
                                </li>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </ul>
            </nav>

            // Collapse button at bottom
            <div class="p-4 border-t border-gray-700">
                <button
                    class="flex items-center w-full px-3 py-2 rounded-lg text-gray-400 hover:bg-gray-700 hover:text-white transition-colors"
                    on:click=move |_| toggle.set(!is_open.get())
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        {move || if is_open.get() {
                            view! { <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 19l-7-7 7-7" /> }.into_any()
                        } else {
                            view! { <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 5l7 7-7 7" /> }.into_any()
                        }}
                    </svg>
                    {move || if is_open.get() {
                        view! { <span class="ml-3 text-sm">"Collapse"</span> }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </button>
            </div>
        </aside>
    }
}

fn nav_icon(icon_type: &str) -> String {
    match icon_type {
        "dashboard" => r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24" class="w-5 h-5"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-4 0a1 1 0 01-1-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 01-1 1h-2z" /></svg>"#.to_string(),
        "theory" => r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24" class="w-5 h-5"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" /></svg>"#.to_string(),
        "practice" => r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24" class="w-5 h-5"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" /></svg>"#.to_string(),
        "projects" => r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24" class="w-5 h-5"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" /></svg>"#.to_string(),
        "settings" => r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24" class="w-5 h-5"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" /></svg>"#.to_string(),
        _ => String::new(),
    }
}
