mod pages;
mod commands;

use dioxus::logger::tracing;
use dioxus::prelude::*;
use pages::not_found::NotFound;
use pages::img_viewer::ImgViewer;
use crate::pages::home::PromptInput;
use crate::pages::home::Home;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/view/:img_name")]
    ImgViewer {img_name: String},
    #[route("/:..route")]
    NotFound {route: Vec<String>},
    // #[route("/blog/:id")]
    // Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const CSS: Asset = asset!("/assets/styles.scss");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: CSS }
        Router::<Route> {}
    }
}

// Home page
// #[component]
// fn Home() -> Element {
//     rsx! {
//         PromptInput {
//             prompt_input: "input",
//             on_submit: || tracing::info!("submitting..."),
//             on_input: |event| tracing::info!("input: {}", event.data().value()),
//             on_keydown: |event| tracing::info!("keydown: {}", event.data().key()),
//             autocomplete: vec!["testing".to_string()],
//             autocomplete_onclick: |autocomplete| tracing::info!("autocomplete: {}", autocomplete)
//         }
//     }
// }
