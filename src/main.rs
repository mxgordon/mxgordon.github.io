use leptos::*;
use mxgordon::App;
use log;

// #[wasm_bindgen(start)]
fn main() {
    // set up logging
    // _ = console_log::init_with_level(log::Level::Debug);
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
