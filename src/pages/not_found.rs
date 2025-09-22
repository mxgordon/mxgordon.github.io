use dioxus::prelude::*;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {  // TODO make this look a bit better
        h1 {
            "Uh oh!"
            br {}
            "We couldn't find that page!"
        }
    }
}
