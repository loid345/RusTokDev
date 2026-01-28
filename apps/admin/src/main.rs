mod api;
mod app;
mod components;
mod pages;
mod providers;

use app::App;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);
    mount_to_body(|| view! { <App /> });
}
