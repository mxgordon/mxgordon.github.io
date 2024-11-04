use leptos::*;
use mxgordon::App;
use log::Level;

fn main() {
    // set up logging
    // _ = console_log::init_with_level(log::Level::Debug);
    console_log::init_with_level(Level::Debug).expect("error initializing log");
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
