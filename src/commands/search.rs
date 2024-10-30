use leptos::{html::P, IntoView};
use leptos::*;

use crate::commands::typewriter::TypeWriter;

use super::about::Intro;

#[derive(Debug, Copy, Clone)]
pub struct Command<'a> {
    name: &'a str,
    syntax: &'a str,
    description: &'a str,
    function: fn(String) -> View,
}

pub static COMMANDS: [Command; 2] = [
    Command {
        name: "help",
        syntax: "help [command]",
        description: "Get help on a command",
        function: |cmd| view!{ <Help cmd={cmd}/>},
    },
    Command {
        name: "intro",
        syntax: "intro",
        description: "Introduction to my website",
        function: |cmd| view!{ <Intro cmd={cmd}/>},
    }
];

pub fn search_commands(cmd: String) -> Vec<Command<'static>> {
    COMMANDS.iter().filter(|c| c.name.contains(&cmd)).cloned().collect()
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
pub fn Help(#[prop(into)] cmd: String) -> impl IntoView {
    view! {
        <TypeWriter html_to_type=help_text()/>
    }
}

#[component]
pub fn CommandNotFound(#[prop(into)] cmd: String) -> impl IntoView {
    let cmd_name = cmd.split_ascii_whitespace().next().unwrap_or_default().to_string();

    view! {
        <TypeWriter html_to_type=view!{<p>{cmd_name}": command not found"</p>} />
    }
}