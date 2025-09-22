use dioxus::prelude::*;
use crate::commands::gallery::*;

#[component]
pub fn ImgViewer(img_name: String) -> Element {
    let img_src = GALLERY.iter()
        .find(|entry| entry.name == img_name)
        .map_or("", |entry| entry.src);

    rsx! {
        div {
            class: "img-view",
            img {
                src: {img_src},
                alt: "description"
            }
        }
    }
}