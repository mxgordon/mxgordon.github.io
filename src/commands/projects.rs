use dioxus::prelude::*;
use crate::commands::typewriter::TypewriterState;
use crate::commands::utils::{check_cmd_args_empty, get_one_cmd_arg, CommandProps, InvalidOption, TypewriterEnd};

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectStatus {
    Complete,
    InProgress,
    Dead
}

#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub github_link: String,
    pub image_link: String,
    pub project_link: Option<String>,
    pub status: ProjectStatus
}

impl Project {
    pub fn make_status(&self, t: TypewriterState) -> Element {
        match self.status {
            ProjectStatus::Complete => rsx! { span {class: "green", {t.t("Complete")}} },
            ProjectStatus::InProgress => rsx! { span {class: "purple", {t.t("In Progress")}} },
            ProjectStatus::Dead => rsx! { span {class: "red", {t.t("Dead")}} },
        }
    }
}

pub fn get_projects() -> Vec<Project> {
    vec! [
        Project {
            name: "WebGL Projects".to_string(),
            description: "A couple different WebGL projects with custom shaders showing different uses such as SVG/2D rendering, mixed rendering, 3D scene rendering, and ray tracing.".to_string(),
            github_link: "https://github.com/mxgordon/WebGL-Projects".to_string(),
            image_link: "https://mxgordon.com/cdn-cgi/image/format=webp/img/v1731358909/webgl-cover_o4fmqe.png".to_string(),
            project_link: Some("/WebGL-Projects/".to_string()),
            status: ProjectStatus::Complete
        },
        Project {
            name: "Conway's (multithreaded) Game of Life".to_string(),
            description: "Conway's game of life, written in C++ using pthread.h and semaphores.h for a custom message passing system that load-balances to ensure optimal multithreading".to_string(),
            github_link: "https://github.com/mxgordon/multithreaded-game-of-life".to_string(),
            image_link: "https://mxgordon.com/cdn-cgi/image/format=webp/img//v1733350692/game-of-life_uremjv.jpg".to_string(),
            project_link: None,
            status: ProjectStatus::Complete
        },
        Project {
            name: "RustyPython".to_string(),
            description: "A python interpreter built in Rust to learn more about both languages, and programming langauges as a whole.".to_string(),
            github_link: "https://github.com/mxgordon/RustyPython".to_string(),
            image_link: "https://mxgordon.com/cdn-cgi/image/format=webp/img/v1733978655/RustlyPythonLogo_cvptuk.png".to_string(),
            project_link: None,
            status: ProjectStatus::InProgress
        },
        Project {
            name: "Personal Website".to_string(),
            description: "I built this website entirely in Rust and Leptos using web-assembly in order to show off my projects and photography. I picked Rust as it's both fast and growing in popularity!".to_string(),
            github_link: "https://github.com/mxgordon/mxgordon.github.io".to_string(),
            image_link: "https://mxgordon.com/cdn-cgi/image/format=webp/img/v1731358909/personal-website-cover_t5tefc.png".to_string(),
            project_link: Some("/".to_string()),
            status: ProjectStatus::Complete
        },
    ]
}

#[component]
pub fn ProjectTile(project: Project, t: TypewriterState) -> Element {
    let project_link2 = project.project_link.clone();

    let link = if project_link2.is_some() {project.project_link.clone().unwrap()} else {project.github_link.clone()};

    rsx! {
        div {
            class: "project-tile",
            h3 { {t.t(&project.name)} }
            p { {t.t("Status: ")} {project.make_status(t.clone())}}
            p { {t.t(&project.description)} }
            a {
                href: link,
                target: "_blank",
                rel: "noopener noreferrer",
                {t.image_alt_loc(&project.image_link, "")}
            }
            div {
                class: "links",
                a {
                    href: project.github_link,
                    target: "_blank",
                    rel: "noopener noreferrer",
                    {t.t("GitHub Repository")}

                }
                if let Some(link) = project_link2 {
                    a {
                        href: link,
                        target: "_blank",
                        rel: "noopener noreferrer",
                        {t.t("Project Link")}
                    }
                }
                {t.send()}
            }
        }
    }
}

pub fn projects_list(projects: Vec<Project>, t: TypewriterState) -> Element {  // make text change with the different filters
    rsx! {
        div {
            h2 { {t.t("My Projects")} }
            p { {t.ts("These are all the projects I have done over the years, please enjoy!")}}
            div { class: "projects",
                for proj in projects {
                    ProjectTile {project: proj.clone(), t: t.clone()}
                }
            }
            TypewriterEnd {t: t.clone()}
        }
    }
}


#[component]
pub fn Projects(props: CommandProps) -> Element {
    let mut arg = None;

    if !check_cmd_args_empty(&props.cmd) {
        arg = get_one_cmd_arg(&props.cmd);

        if arg.is_none() {  // There was more than one argument
            return rsx! {
                InvalidOption {..props}
            }
        }
    }

    if let Some(arg) = arg {
        let status_arg = match arg {
            "complete" => ProjectStatus::Complete,
            "in-progress" => ProjectStatus::InProgress,
            "dead" => ProjectStatus::Dead,
            _ => return rsx! {
                InvalidOption {..props}
            }
        };

        return projects_list(get_projects().into_iter().filter(|proj| proj.status == status_arg).collect(), props.typewriter_state);
    }

    projects_list(get_projects(), props.typewriter_state)
}