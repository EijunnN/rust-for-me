mod app;
mod components;
mod i18n;
mod models;
mod pages;
mod services;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
