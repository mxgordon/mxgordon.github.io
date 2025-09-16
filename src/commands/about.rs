use dioxus::prelude::*;
use crate::commands::utils::{check_cmd_args_empty, CommandProps, InvalidOption};

pub const EMAIL_ICON: Asset = asset!("/assets/icons/email.png");
pub const GITHUB_ICON: Asset = asset!("/assets/icons/github.png");
pub const LINKEDIN_ICON: Asset = asset!("/assets/icons/linkedin.png");
pub const RESUME_ICON: Asset = asset!("/assets/icons/resume.png");
pub const RESUME_PDF: Asset = asset!("/assets/documents/Max_Gordon_resume.pdf");

#[component]
pub fn About(props: CommandProps) -> Element {
    if !check_cmd_args_empty(&props.cmd) {
        return rsx! {
            InvalidOption {
                ..props
            }
        }
    }

    let mut t = props.typewriter_state;

    let rtn = rsx! {
        p {
            h2 { {t.t("About Me")} }
            p {
                {t.t("I am a computer science BS/MS student at Worcester Polytechnic University (")}
                a { href: "https://www.wpi.edu/", target: "_blank", rel: "noopener noreferrer", {t.t("WPI")} }
                {t.t(") in my senior year. I have a strong interest in systems-level programming and financial technology. In high school, I was on a FIRST robotics competition (")}
                a { href: "https://www.firstinspires.org/robotics/frc", target: "_blank", rel: "noopener noreferrer", {t.t("FRC")} }
                {t.t(") team, where I discovered my passions for programming, electronics, and additive manufacturing.")}
            }
            p {
                {t.t("Last summer, I interned at ")}
                a { href: "https://www.citizensbank.com/homepage.aspx", target: "_blank", rel: "noopener noreferrer", {t.t("Citizens Bank")} }
                {t.t(" where I worked on the DevOps team. During my internship, I onboarded 3 new services and migrated several CI/CD pipelines to use AWS EKS instead of Redhat Openshift.")}
                {t.t("The year before, I interned at ")}
                a { href: "https://www.finra.org", target: "_blank", rel: "noopener noreferrer", {t.t("FINRA")} }
                {t.t(" where I developed my interest for financial technology, and learned about it from the regulatory side.")}
                }
            p {
                {t.t("Last fall semester (2024), I spent 2 months studying abroad in Panama City, working with ")}
                a { href: "https://www.simpleaf.earth/", target: "_blank", rel: "noopener noreferrer", {t.t("simple af")} }
                {t.t(", a growing sustainable design studio. Their focus is manufacturing fabric goods from used fabrics such as billboards and clothes, with the goal of expanding the circular economy in Latin America.")}
            }
            p {
                {t.t("In my freetime I like to work on my car and shoot film photography (try the `")}
                span { class: "orange", {t.t("gallery")} }
                {t.t("` command). Additionally, I enjoy 3D printing and tinkering with my printer.")}
            }
            p { {t.t("For more information about me, check out these links:")} }

            div {
                class: "about-links",
                a { href: "https://github.com/mxgordon", target: "_blank", rel: "noopener noreferrer", p { {t.image(GITHUB_ICON)} {t.t("GitHub")} } }
                a { href: "https://www.linkedin.com/in/max-gordon-533423221/", target: "_blank", rel: "noopener noreferrer", p { {t.image(LINKEDIN_ICON) } {t.t("LinkedIn")} } }
                a { href: RESUME_PDF, target: "_blank", rel: "noopener noreferrer", p { {t.image(RESUME_ICON) } {t.t("Resume")} } }
                a { href: "mailto:mgordon291us@gmail.com", target: "_blank", rel: "noopener noreferrer", p { {t.image(EMAIL_ICON) } {t.t("Email")} } }

            }
        }

    };

    t.finish();

    rtn
}