use crate::global_attributes::class;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use dioxus::prelude::*;
use dioxus_sdk::utils::timing::{use_interval, UseInterval};
use crate::commands::typewriter::TypewriterState;

#[component]
pub fn intro_html() -> Element {
    let mut typw = TypewriterState::new();

    let rtn = rsx!{
        p { class: "typing-parent",
            p { class: {typw.cc("ascii-art")}, {typw.t("Hi, I'm...")} }
            span { class: "orange ascii-art",
                p { class: {typw.c()}, {typw.t(" __  __               _____               _")} }
                p { class: {typw.c()}, {typw.t("|  \\/  |             / ____|             | |")} }
                p { class: {typw.c()}, {typw.t("| \\  / | __ ___  __ | |  __  ___  _ __ __| | ___  _ __  ")} }
                p { class: {typw.c()}, {typw.t("| |\\/| |/ _` \\ \\/ / | | |_ |/ _ \\| '__/ _` |/ _ \\| '_ \\ ")}}
                p { class: {typw.c()}, {typw.t("| |  | | (_| |>  <  | |__| | (_) | | | (_| | (_) | | | |")} }
                p { class: {typw.c()}, {typw.t("|_|  |_|\\__,_/_/\\_\\  \\_____|\\___/|_|  \\__,_|\\___/|_| |_|")}}
            }
            p { class: {typw.cn(3)}, {typw.t("Instead of navigating with buttons and hyperlinks, on my page you will use terminal commands. Start with `")} span { class: "orange", {typw.t("help")} } {typw.t("` for the list of commands and syntax!")} }
            p { class: {typw.cn(5)},
                {typw.t("Made using ")}
                a {
                    href: "https://www.rust-lang.org/",
                    rel: "noopener noreferrer",
                    target: "_blank",
                    {typw.t("Rust")}
                }
                {typw.t(" 🦀 and ")}
                a {
                    href: "https://dioxuslabs.com/",
                    rel: "noopener noreferrer",
                    target: "_blank",
                    {typw.t("Dioxus")}
                }
                {typw.t("!")}
            }
        }
    };

    typw.typewriter();

    rtn
}
