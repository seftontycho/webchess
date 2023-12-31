mod algorithm;
mod app;
mod game;
mod opponent;

use app::App;
use leptos::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    log!("csr mode - mounting to body");

    mount_to_body(|cx| {
        view! { cx, <App /> }
    });
}
