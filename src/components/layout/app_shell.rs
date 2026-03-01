use leptos::prelude::*;
use leptos::ev;
use super::sidebar_nav::SidebarNav;
use super::top_bar::TopBar;
use crate::components::playground::playground_drawer::PlaygroundDrawer;
use crate::components::playground::playground_context::use_playground;

/// The main application shell that wraps all page content with
/// a sidebar navigation and top bar.
#[component]
pub fn AppShell(children: Children) -> impl IntoView {
    let (sidebar_open, set_sidebar_open) = signal(true);
    let pg = use_playground();

    // Global keyboard shortcuts
    let on_keydown = move |e: ev::KeyboardEvent| {
        // Ctrl+Shift+P → toggle playground
        if e.ctrl_key() && e.shift_key() && e.key() == "P" {
            e.prevent_default();
            pg.is_open.set(!pg.is_open.get());
        }
        // Escape → close playground
        if e.key() == "Escape" && pg.is_open.get() {
            pg.is_open.set(false);
        }
    };

    view! {
        <div
            class="flex h-screen overflow-hidden bg-gray-50 dark:bg-gray-900"
            on:keydown=on_keydown
        >
            <SidebarNav is_open=sidebar_open toggle=set_sidebar_open />
            <div class="flex flex-col flex-1 overflow-hidden">
                <TopBar on_toggle_sidebar=move |_| set_sidebar_open.set(!sidebar_open.get()) />
                <main class="flex-1 overflow-y-auto p-6 bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-100">
                    {children()}
                </main>
            </div>
            <PlaygroundDrawer />
        </div>
    }
}
