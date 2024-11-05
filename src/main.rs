use leptos::*;
use mxgordon::App;
use log;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use tracing_wasm::{WASMLayer, WASMLayerConfig};
use web_sys::js_sys::wasm_bindgen;

// #[wasm_bindgen(start)]
fn main() {
    // set up logging
    // _ = console_log::init_with_level(log::Level::Debug);
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");
    console_error_panic_hook::set_once();

    // let config = WASMLayerConfig::new(Level::TRACE);
    // let wasm_layer = WASMLayer::new(config);
    // let subscriber = FmtSubscriber::builder()
    //     .with_max_level(Level::TRACE)
    //     .finish()
    //     .with(wasm_layer);

    // tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    console_error_panic_hook::set_once();

    // Add this line:
    tracing_wasm::set_as_global_default();

    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
