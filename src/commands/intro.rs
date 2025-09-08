use dioxus::prelude::*;
use crate::commands::utils::CommandProps;

#[component]
pub fn Intro(props: CommandProps) -> Element {
    let mut t = props.typewriter_state;

    let rtn = rsx!{
        p { class: "typing-parent",
            p { class: "ascii-art", {t.t("Hi, I'm...")} }
            span { class: "orange ascii-art",
                p { {t.t(" __  __               _____               _")} }
                p { {t.t("|  \\/  |             / ____|             | |")} }
                p { {t.t("| \\  / | __ ___  __ | |  __  ___  _ __ __| | ___  _ __  ")} }
                p { {t.t("| |\\/| |/ _` \\ \\/ / | | |_ |/ _ \\| '__/ _` |/ _ \\| '_ \\ ")}}
                p { {t.t("| |  | | (_| |>  <  | |__| | (_) | | | (_| | (_) | | | |")} }
                p { {t.t("|_|  |_|\\__,_/_/\\_\\  \\_____|\\___/|_|  \\__,_|\\___/|_| |_|")}}
            }
            p { {t.t("Instead of navigating with buttons and hyperlinks, on my page you will use terminal commands. Start with `")} span { class: "orange", {t.t("help")} } {t.t("` for the list of commands and syntax!")} }
            p {
                {t.t("Made using ")}
                a {
                    href: "https://www.rust-lang.org/",
                    rel: "noopener noreferrer",
                    target: "_blank",
                    {t.t("Rust")}
                }
                {t.t(" 🦀 and ")}
                a {
                    href: "https://dioxuslabs.com/",
                    rel: "noopener noreferrer",
                    target: "_blank",
                    {t.t("Dioxus")}
                }
                {t.t("!")}
            }
        }
    };

    t.set_on_finished_callback();

    rtn
}
