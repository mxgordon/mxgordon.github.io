#![feature(fn_traits)]

mod pages;
mod commands;

use dioxus::prelude::*;
use pages::not_found::NotFound;
use pages::img_viewer::ImgViewer;
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
        document::Title { "Max Gordon" }
        Router::<Route> {}
    }
}

