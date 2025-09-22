use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use crate::commands::typewriter::TypewriterState;
use crate::commands::utils::{get_command, CommandProps, COMMANDS};

pub fn help_text_1_arg(cmd_name: String, t: TypewriterState) -> Element {
    let cmd = get_command(&cmd_name);

    if let Some(cmd) = cmd {
        rsx! {
            p {
                p {{t.t(cmd.name)} {t.t(": ")} {t.t(cmd.syntax)}}
                p {{t.te(cmd.description)}}
            }
        }
    } else {
        rsx! {
            p {{t.t("help: ")} {t.t(&cmd_name)} {t.te(": command not found")}}
        }
    }
}

#[component]
pub fn Help(props: CommandProps) -> Element {
    let cmd_split = props.cmd
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    let t = props.typewriter_state;

    info!("{:?}", cmd_split);

    match cmd_split.len() {
        1 => rsx! {
            p {
                p {{t.t("MAX bash, version 0.0.1")}}
                p {{t.t("These are all the commands supported on this platform. To learn more about a command use `help [command]`.")}}
                for cmd in COMMANDS.iter() {
                    p {{t.t("- ")} {t.t(cmd.name)}}
                }
                {t.end()}
            }
        },
        2 => help_text_1_arg(cmd_split[1].clone(), t),
        n => rsx! {
            p {{t.t("help: expected 0 or 1 arguments, received ")} {t.te(n.to_string().as_str())}}
        }
    }
}