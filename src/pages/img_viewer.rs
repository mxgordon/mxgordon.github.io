use leptos::*;
use leptos_dom::helpers::location;
use leptos_router::*;

use crate::commands::gallery::{get_gallery, Gallery, GalleryEntry};

use leptos::logging::log;

#[derive(Params, PartialEq)]
struct ImgParams {
    name: String
}

#[component]
pub fn ImgViewer() -> impl IntoView {
    let params = use_params::<ImgParams>();
    let name = move || params.with(|params| 
        params.as_ref()
            .map(|params| params.name.clone())
            .unwrap_or_default());

    let gallery = get_gallery();

    let loc = location();

    let redir = gallery.iter().find(|entry| entry.name == name());

    if let Some(redir) = redir {
        let _ = loc.set_href(redir.src);
    }


    log!("{}", name());

    view! {
        // <img src={gallery.iter().find(|entry| entry.name == name()).unwrap_or(&GalleryEntry{src:"", name:"",description:""}).src} alt="placeholder" />
        <p>Nurrr</p>
    }
}