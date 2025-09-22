use dioxus::prelude::*;
use levenshtein::levenshtein;
use crate::commands::about::About;
use crate::commands::gallery::{Gallery};
use crate::commands::help::Help;
use crate::commands::intro::Intro;
use crate::commands::projects::Projects;
use crate::commands::typewriter::TypewriterState;

#[derive(Debug, Copy, Clone)]
pub struct Command<'a> {
    pub name: &'a str,
    pub syntax: &'a str,
    pub description: &'a str,
    pub function: fn(CommandProps) -> Element,
}

pub static COMMANDS: [Command; 5] = [
    Command {
        name: "intro",
        syntax: "intro",
        description: "The introduction to my website.",
        function: Intro,
    },
    Command {
        name: "about",
        syntax: "about",
        description: "The 'about me' section; come learn more about my career, my hobbies, and my webste!",
        function: About,
    },
    Command {
        name: "gallery",
        syntax: "gallery <image>",
        description: "My personal gallery of film photography! ",
        function: Gallery,
    },
    Command {
        name: "projects",
        syntax: "projects [filter]",
        description: "All my public projects, past and present. You can filter by status, options are `complete`, `in-progress`, and `dead`",
        function: Projects,
    },
    Command {
        name: "help",
        syntax: "help [command]",
        description: "Get help on a command.",
        function: Help,
    },

];

pub fn search_commands(cmd: &str) -> Vec<Command<'static>> {
    if cmd.is_empty() {
        return vec![];
    }

    let cmd_name = cmd.split_whitespace().next().unwrap_or_default();
    COMMANDS
        .iter()
        .filter(|c| c.name.starts_with(cmd_name))
        .cloned()
        .collect()
}

pub fn get_command(cmd: &str) -> Option<Command<'static>> {
    let possible_commands = search_commands(cmd);
    if possible_commands.is_empty() || possible_commands[0].name != cmd {
        None
    } else {
        Some(possible_commands[0])
    }
}

pub fn get_one_cmd_arg(cmd: &String) -> Option<&str> {
    let mut splits = cmd.split_whitespace();

    if splits.clone().count() != 2 {
        return None;
    }

    splits.nth(1)
}

pub fn check_cmd_args_empty(cmd: &str) -> bool {
    cmd.split_whitespace().count() == 1
}

#[derive(Clone, PartialEq, Props)]
pub struct CommandProps {
    pub cmd: String,
    pub typewriter_state: TypewriterState
}

impl CommandProps {
    pub fn new(cmd: String, cmd_number: i32) -> Self {
        Self { cmd, typewriter_state: TypewriterState::new(cmd_number) }
    }
}

#[component]
pub fn CommandNotFound(props: CommandProps) -> Element {
    let cmd_name = props.cmd.split_ascii_whitespace().next().unwrap_or_default().to_string();
    let suggestion = COMMANDS.iter().map(|c| (c.name, levenshtein(&cmd_name, c.name))).fold(("", usize::MAX), |prev, next| {
        if next.1 < prev.1 { next } else { prev }
    });

    let t = props.typewriter_state;

    rsx! {
        p {
            {t.t(&cmd_name)}{t.t(": command not found. Did you mean: `")}
            span { class: "orange", {t.t(suggestion.0)}}
            {t.te("`?")}
        }
    }
}

#[component]
pub fn InvalidOption(props: CommandProps) -> Element {
    let t = props.typewriter_state;

    rsx! {
        p {
            {t.t(&props.cmd)}{t.te(": invalid option")}
        }
    }
}

#[component]
pub fn TypewriterEnd(t: TypewriterState) -> Element {
    t.end()
}