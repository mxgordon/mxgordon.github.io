use html::{Div, P};
use leptos::*;
use svg::G;

use crate::commands::{typewriter::TypeWriter, utils::{check_cmd_args_empty, InvalidOption}};

#[component]
pub fn GalleryImage(#[prop(into)] src: String, #[prop(into)] description: String) -> impl IntoView {
    let src = format!("/cdn-cgi/image/format=webp/{}", src);

    view! {
        // <img src=src alt=alt />
        <div class="gallery-item">
            <a href=src.clone() target="_blank" rel="noopener noreferrer">
                <img src=src alt=description.clone() />
            </a>
            <p>{description}</p>
        </div>
    }
}

#[derive(Clone)]
pub struct GalleryEntry<'a> {
    name: &'a str,
    src: &'a str,
    description: &'a str,
}

pub fn get_gallery<'a>() -> Vec<GalleryEntry<'a>> {
    vec! [
        GalleryEntry {
            name: "beach_sunset",
            src: "https://res.cloudinary.com/dtz40humd/image/upload/v1730920374/img2_003_result_ajtqzg.jpg",
            description: "A photo of a tree with a sunset in the background",
        },
        GalleryEntry {
            name: "cameras",
            src: "https://res.cloudinary.com/dtz40humd/image/upload/v1730920375/img068_result_nmmeuf.jpg", 
            description: "A photo of a tree with a sunset in the background"
        },
        GalleryEntry {
            name: "worcester",
            src: "https://res.cloudinary.com/dtz40humd/image/upload/v1730920375/img064_result_ty9x29.jpg", 
            description: "A photo of a tree with a sunset in the background"
        },
        GalleryEntry {
            name: "rainbow_mountain",
            src: "https://res.cloudinary.com/dtz40humd/image/upload/v1730920374/img2_041_result_yfeiql.jpg", 
            description: "A photo of a tree with a sunset in the background"
        },
        GalleryEntry {
            name: "red_valley",
            src: "https://res.cloudinary.com/dtz40humd/image/upload/v1730920374/img2_042_result_kcgxsi.jpg", 
            description: "A photo of a tree with a sunset in the background"
        },
        GalleryEntry {
            name: "macchu_picchu",
            src: "https://res.cloudinary.com/dtz40humd/image/upload/v1730938313/img2_026_result2_tt1qc7.jpg", 
            description: "A photo of a tree with a sunset in the background"
        },
        GalleryEntry {
            name: "cusco_stree",
            src: "https://res.cloudinary.com/dtz40humd/image/upload/v1730920375/img2_047_result_jumilw.jpg", 
            description: "A photo of a tree with a sunset in the background"
        },
        GalleryEntry {
            name: "cusco_skyline",
            src: "https://res.cloudinary.com/dtz40humd/image/upload/v1730920375/img2_045_result_wh7zm3.jpg", 
            description: "A photo of a tree with a sunset in the background"
        },
        GalleryEntry {
            name: "long_beach",
            src: "https://res.cloudinary.com/dtz40humd/image/upload/v1730920373/img2_001_result_qbbbag.jpg", 
            description: "A photo of a tree with a sunset in the background"
        },

    ]
}

pub fn gallery_html<'a>() -> HtmlElement<Div> {
    view! {
        <div>
            <h2>My Gallery</h2>
            <p>"I enjoy shooting film photography in my freetime. I mainly shoot in color, but occasionally I'll shoot in black & white, as its easier to enlarge and such. Most of these photos were shot on my Minolta XG-9."</p>
            <p>"Here are some of my favorite shots!"</p>
            <div class="gallery">
                <For each=move || get_gallery().into_iter() key=|entry| entry.name.to_string() children=move |entry| {
                    view! {
                        <GalleryImage src=entry.src description=entry.description />
                    }
                } />
            </div>
        </div>
    }
}

#[component]
pub fn Gallery(#[prop()] cmd: String, #[prop(default=Box::new(|| ()))] on_finished: Box<dyn Fn() + 'static>) -> impl IntoView {
    if !check_cmd_args_empty(&cmd) {
        return view! {
            <InvalidOption cmd=cmd on_finished=on_finished />
        }
    }
    
    view! {
        <TypeWriter html_to_type=gallery_html() callback=on_finished />
    }
}