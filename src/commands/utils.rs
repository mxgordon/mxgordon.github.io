use leptos::*;
use crate::commands::typewriter::TypeWriter;


pub fn check_cmd_args_empty(cmd: &String) -> bool {
    cmd.split_whitespace().count() == 1
}

#[component]
pub fn CommandNotFound(#[prop(into)] cmd: String, #[prop()] on_finished: Box<dyn Fn() + 'static>) -> impl IntoView {
    let cmd_name = cmd.split_ascii_whitespace().next().unwrap_or_default().to_string();

    view! {
        <TypeWriter html_to_type=view!{<p>{cmd_name}": command not found"</p>} callback=on_finished />
    }
}

#[component]
pub fn InvalidOption(#[prop(into)] cmd: String, #[prop()] on_finished: Box<dyn Fn() + 'static>) -> impl IntoView {
    view! {
        <TypeWriter html_to_type=view!{<p>{cmd}": invalid option"</p>} callback=on_finished />
    }
}