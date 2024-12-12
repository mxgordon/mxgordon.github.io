use html::Div;
use leptos::*;

use crate::commands::{typewriter::TypeWriter, utils::{check_cmd_args_empty, get_one_cmd_arg, InvalidOption, InvalidOptionProps}};

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectStatus {
    Complete,
    InProgress,
    Dead
}

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub github_link: String,
    pub image_link: String,
    pub project_link: Option<String>,
    pub status: ProjectStatus
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
pub fn ProjectTile(#[prop()] project: Project) -> impl IntoView {
    let project_link2 = project.project_link.clone();

    let link = if project_link2.is_some() {project.project_link.clone().unwrap()} else {project.github_link.clone()};

    let status = match project.status {
        ProjectStatus::Complete => view! {<span class="green">"Complete"</span>},
        ProjectStatus::InProgress => view! {<span class="purple">"In Progress"</span>},
        ProjectStatus::Dead => view! {<span class="red">"Dead"</span>},
    };

    view! {
        <div class="project-tile">
            <h3>{project.name}</h3>
            <p class="status">"Status: "{status}</p>
            <p>{project.description}</p>
            <a href={link} target="_blank" rel="noopener noreferrer">
                <img src={project.image_link} alt=""/>
            </a>
            <div class="links" >
                <a href={project.github_link} target="_blank" rel="noopener noreferrer" >"GitHub Repository"</a>
                <Show when={move || project_link2.is_some()} >
                    <a href={project.project_link.clone().unwrap()} target="_blank" rel="noopener noreferrer">"Project Link"</a>
                </Show>
            </div>
        </div>
    }
}

pub fn projects_html_with_filter(filter: ProjectStatus) -> HtmlElement<Div> {
    let projects = get_projects();

    projects_html(projects.into_iter().filter(|proj| proj.status == filter).collect())
}

pub fn projects_html(projects: Vec<Project>) -> HtmlElement<Div> {  // make text change with the different filters

    view! {
        <div>
            <h2>"My Projects"</h2>
            <p>"These are all the projects I have done over the years, please enjoy!"</p>
            <div class="projects">
                <For each=move || projects.clone().into_iter() key=|p| p.name.clone() children=move |p| {
                    view! {<ProjectTile project=p  />}
                }/>
            </div>
        </div>
    }
}


#[component]
pub fn Projects(#[prop()] cmd: String, #[prop(default=Box::new(|| ()))] on_finished: Box<dyn Fn() + 'static>) -> impl IntoView {
    let mut arg = None;
    
    if !check_cmd_args_empty(&cmd) {
        arg = get_one_cmd_arg(&cmd);

        if arg.is_none() {  // There was more than one argument
            return view! {
                <InvalidOption cmd=cmd on_finished=on_finished />
            }
        }
    }

    if let Some(arg) = arg {
        let status_arg = match arg {
            "complete" => ProjectStatus::Complete,
            "in-progress" => ProjectStatus::InProgress,
            "dead" => ProjectStatus::Dead,
            _ => return view! {
                <InvalidOption cmd=cmd on_finished=on_finished />
            }
        };

        return view! {
            <TypeWriter html_to_type=projects_html_with_filter(status_arg) callback=on_finished  />
        };
    }
    
    view! {
        <TypeWriter html_to_type=projects_html(get_projects()) callback=on_finished  />
    }
}