// use leptos_router::params::Params;
// use leptos::prelude::ElementChild;
use leptos::*;
// use leptos::prelude::{ClassAttribute, With};
// use leptos_router::hooks::use_params;
// use crate::commands::gallery::{get_gallery, GalleryEntry};

#[derive(PartialEq)]
struct ImgParams {
    name: String
}

#[component]
pub fn ImgViewer() -> impl IntoView {
    // let params = use_params::<ImgParams>();
    // let name = move || params.with(|params| 
    //     params.as_ref()
    //         .map(|params| params.name.clone())
    //         .unwrap_or_default());
    // 
    // let gallery = get_gallery();
    // 
    // view! {
    //     <div class="img-view">
    //         <img src={gallery.iter().find(|entry| entry.name == name()).unwrap_or(&GalleryEntry{src:"", name:"",description:""}).src} alt="placeholder" />
    //     </div>
    // }
}