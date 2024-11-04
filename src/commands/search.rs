use leptos::{html::P, IntoView};
use leptos::*;

use crate::commands::about::check_cmd_args_empty;
use crate::commands::typewriter::TypeWriter;

use super::about::Intro;

#[derive(Debug, Copy, Clone)]
pub struct Command<'a> {
    pub name: &'a str,
    pub syntax: &'a str,
    pub description: &'a str,
    pub function: fn(String, Box<dyn Fn() + 'static>) -> View,
}

pub static COMMANDS: [Command; 2] = [
    Command {
        name: "help",
        syntax: "help [command]",
        description: "Get help on a command",
        function: |cmd, on_finished| view!{ <Help cmd={cmd} on_finished=on_finished/>},
    },
    Command {
        name: "intro",
        syntax: "intro",
        description: "Introduction to my website",
        function: |cmd, on_finished| view!{ <Intro cmd={cmd} on_finished=on_finished/>},
    }
];

pub fn search_commands(cmd: String) -> Vec<Command<'static>> {
    let cmd_name = cmd.split_whitespace().next().unwrap_or_default();
    COMMANDS.iter().filter(|c| c.name.contains(&cmd_name)).cloned().collect()
}

pub fn get_command(cmd: String) -> Option<Command<'static>> {
    let cmd_name = cmd.split_whitespace().next().unwrap_or_default();
    COMMANDS.iter().find(|c| c.name == cmd_name).cloned()
}

pub fn help_text() -> HtmlElement<P> {
    view! {
        <p>
            <p>"MAX bash, version 0.0.1"</p>
            <p>"These are all the commands supported on this platform. To learn more about a command use `help [command]`."</p>
            <For each=|| COMMANDS.iter() key=|cmd| cmd.name children=|cmd| view!{<p>" - "{cmd.name}</p>} />
        </p>
    }
}

#[component]
pub fn Help(#[prop(into)] cmd: String, #[prop()] on_finished: Box<dyn Fn() + 'static>) -> impl IntoView {
    if !check_cmd_args_empty(&cmd) {
        return view! {
            <InvalidOption cmd=cmd on_finished=on_finished />
        }
    }
    
    view! {
        <TypeWriter html_to_type=help_text() callback=on_finished/>
    }
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