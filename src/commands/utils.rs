use dioxus::prelude::*;
use levenshtein::levenshtein;
use crate::commands::gallery::{gallery_html, Gallery};

#[derive(Debug, Copy, Clone)]
pub struct Command<'a> {
    pub name: &'a str,
    pub syntax: &'a str,
    pub description: &'a str,
    pub function: fn(CommandProps) -> Element,
}

pub static COMMANDS: [Command; 1] = [
    // Command {
    //     name: "intro",
    //     syntax: "intro",
    //     description: "The introduction to my website.",
    //     function: |cmd, on_finished,| view! { <Intro cmd={cmd} on_finished=on_finished/>},
    // },
    // Command {
    //     name: "about",
    //     syntax: "about",
    //     description: "The 'about me' section; come learn more about my career, my hobbies, and my webste!",
    //     function: |cmd, on_finished,| view! { <About cmd={cmd} on_finished=on_finished/>},
    // },
    Command {
        name: "gallery",
        syntax: "gallery",
        description: "My personal gallery of film photography! ",
        function: Gallery,
    },
    // Command {
    //     name: "projects",
    //     syntax: "projects [filter]",
    //     description: "All my public projects, past and present. You can filter by status, options are `complete`, `in-progress`, and `dead`",
    //     function: |cmd, on_finished| view! { <Projects cmd={cmd} on_finished=on_finished />}
    // },
    // Command {
    //     name: "help",
    //     syntax: "help [command]",
    //     description: "Get help on a command.",
    //     function: |cmd, on_finished,| view! { <Help cmd={cmd} on_finished=on_finished/>},
    // },

];

pub fn get_one_cmd_arg(cmd: &String) -> Option<&str> {
    let mut splits = cmd.split_whitespace();

    if splits.clone().count() != 2 {
        return None;
    }

    splits.nth(1)
}

pub fn check_cmd_args_empty(cmd: &String) -> bool {
    cmd.split_whitespace().count() == 1
}

#[derive(Clone, PartialEq)]
pub struct CommandProps {
    cmd: String,
    on_finished: fn(),
}

#[component]
pub fn CommandNotFound(props: CommandProps) -> Element {
    let cmd_name = props.cmd.split_ascii_whitespace().next().unwrap_or_default().to_string().to_lowercase();
    let suggestion = COMMANDS.iter().map(|c| (c.name, levenshtein(&props.cmd, c.name))).fold(("", usize::MAX), |prev, next| {
        if next.1 < prev.1 { next } else { prev }
    });

    rsx! {
        p {
            {cmd_name}": command not found. Did you mean: `"
            span { class: "orange", {suggestion.0}}
            "`?"
        }
    }

    // view! {
    //     <TypeWriter html_to_type=view!{<p>{cmd_name}": command not found. Did you mean: `"<span class="orange">{suggestion.0}</span>"`?"</p>} callback=on_finished />
    // }
}

#[component]
pub fn InvalidOption(props: CommandProps) -> Element {
    rsx! {
        p {
            {props.cmd}": invalid option"
        }   
    }
    // view! {
    //     <TypeWriter html_to_type=view!{<p>{cmd}": invalid option"</p>} callback=on_finished />
    // }
}