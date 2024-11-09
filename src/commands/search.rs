use leptos::*;
use leptos::{IntoView, html::P};

use crate::commands::typewriter::TypeWriter;

use super::about::*;
use super::gallery::Gallery;

#[derive(Debug, Copy, Clone)]
pub struct Command<'a> {
    pub name: &'a str,
    pub syntax: &'a str,
    pub description: &'a str,
    pub function: fn(String, Box<dyn Fn() + 'static>) -> View,
}

pub static COMMANDS: [Command; 4] = [
    Command {
        name: "help",
        syntax: "help [command]",
        description: "Get help on a command",
        function: |cmd, on_finished,| view! { <Help cmd={cmd} on_finished=on_finished/>},
    },
    Command {
        name: "intro",
        syntax: "intro",
        description: "Introduction to my website",
        function: |cmd, on_finished,| view! { <Intro cmd={cmd} on_finished=on_finished/>},
    },
    Command {
        name: "about",
        syntax: "about",
        description: "The about me section, come learn more about my career, my hobbies, and my webste!",
        function: |cmd, on_finished,| view! { <About cmd={cmd} on_finished=on_finished/>},
    },
    Command {
        name: "gallery",
        syntax: "gallery",
        description: "My personal gallery of film photography! ",
        function: |cmd, on_finished,| view! { <Gallery cmd={cmd} on_finished=on_finished/>},
    },

];

pub fn search_commands(cmd: String) -> Vec<Command<'static>> {
    if cmd.is_empty() {
        return vec![];
    }

    let cmd_name = cmd.split_whitespace().next().unwrap_or_default();
    COMMANDS
        .iter()
        .filter(|c| c.name.contains(&cmd_name))
        .cloned()
        .collect()
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

pub fn help_text_1_arg(cmd_name: String) -> HtmlElement<P> {
    let cmd = get_command(cmd_name.to_string());

    if let Some(cmd) = cmd {
        view! {
            <p>
                <p>{cmd.name}": "{cmd.syntax}</p>
                <p>{cmd.description}</p>
            </p>
        }
    } else {
        view! {
            <p>"help: "{cmd_name}": command not found"</p>
        }
    }
}

#[component]
pub fn Help(
    #[prop(into)] cmd: String,
    #[prop()] on_finished: Box<dyn Fn() + 'static>,
) -> impl IntoView {
    let cmd_split = cmd
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    match cmd_split.len() {
        1 => view! {
            <TypeWriter html_to_type=help_text() callback=on_finished/>
        },
        2 => view! {
            <TypeWriter html_to_type=help_text_1_arg(cmd_split[1].clone()) callback=on_finished/>
        },
        n => {
            view! {
                <TypeWriter html_to_type=view!{<p>"help: expected 0 or 1 arguments, received "{n}</p>} callback=on_finished/>
            }
        }
    }
}
