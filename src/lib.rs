use leptos::prelude::AddAnyAttr;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_router::components::{Route, Router, Routes};
use pages::img_viewer::ImgViewer;

mod pages;
mod commands;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn app() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html {..} lang="en" dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="Max Gordon"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            <Routes fallback=|| view!{ <NotFound/> }>
                <Route path=path!("/") view=Home/>
                <Route path=path!("/view/:name") view=ImgViewer/>
            </Routes>
        </Router>
    }
}
