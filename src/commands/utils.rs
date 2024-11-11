use leptos::*;
use crate::commands::{search::COMMANDS, typewriter::TypeWriter};
use levenshtein::levenshtein;


pub fn check_cmd_args_empty(cmd: &String) -> bool {
    cmd.split_whitespace().count() == 1
}

#[component]
pub fn CommandNotFound(#[prop(into)] cmd: String, #[prop()] on_finished: Box<dyn Fn() + 'static>) -> impl IntoView {
    let cmd_name = cmd.split_ascii_whitespace().next().unwrap_or_default().to_string().to_lowercase();
    let suggestion = COMMANDS.iter().map(|c| (c.name, levenshtein(&cmd, c.name))).fold(("", usize::MAX), |prev, next| {
        if next.1 < prev.1 { next } else { prev }
    });

    view! {
        <TypeWriter html_to_type=view!{<p>{cmd_name}": command not found. Did you mean: `"<span class="orange">{suggestion.0}</span>"`?"</p>} callback=on_finished />
    }
}

#[component]
pub fn InvalidOption(#[prop(into)] cmd: String, #[prop()] on_finished: Box<dyn Fn() + 'static>) -> impl IntoView {
    view! {
        <TypeWriter html_to_type=view!{<p>{cmd}": invalid option"</p>} callback=on_finished />
    }
}