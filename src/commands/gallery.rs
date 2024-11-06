use html::P;
use leptos::*;

use crate::commands::{typewriter::TypeWriter, utils::{check_cmd_args_empty, InvalidOption}};

pub fn gallery_html() -> HtmlElement<P> {
    view! {
        <p>"🚧 Under construction 🚧"</p>
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