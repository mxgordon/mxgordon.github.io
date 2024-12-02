use html::Div;
use leptos::*;

use crate::commands::{typewriter::TypeWriter, utils::{check_cmd_args_empty, InvalidOption}};

#[derive(Clone)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub github_link: String,
    pub image_link: String,
    pub project_link: Option<String>,

}
pub fn get_projects() -> Vec<Project> {
    vec! [
        Project {
            name: "WebGL Projects".to_string(),
            description: "A couple different WebGL projects with custom shaders showing different uses such as SVG/2D rendering, mixed rendering, 3D scene rendering, and ray tracing.".to_string(),
            github_link: "https://github.com/mxgordon/WebGL-Projects".to_string(),
            image_link: "https://mxgordon.com/cdn-cgi/image/format=webp/img/v1731358909/webgl-cover_o4fmqe.png".to_string(),
            project_link: Some("/WebGL-Projects/".to_string()),
        },
        Project {
            name: "Personal Website".to_string(),
            description: "I built this website entirely in Rust and Leptos using web-assembly in order to show off my projects and photography. I picked Rust as it's both fast and growing in popularity!".to_string(),
            github_link: "https://github.com/mxgordon/mxgordon.github.io".to_string(),
            image_link: "https://mxgordon.com/cdn-cgi/image/format=webp/img/v1731358909/personal-website-cover_t5tefc.png".to_string(),
            project_link: Some("/".to_string()),
        }
    ]
}

#[component]
pub fn ProjectTile(#[prop()] project: Project) -> impl IntoView {
    let project_link2 = project.project_link.clone();

    let link = if project_link2.is_some() {project.project_link.clone().unwrap()} else {project.github_link.clone()};

    view! {
        <div class="project-tile">
            <h3>{project.name}</h3>
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

pub fn projects_html() -> HtmlElement<Div> {
    let projects = get_projects();

    view! {
        <div>
            <h2>"My Projects"</h2>
            <p>"These are all the projects I have finished over the years, please enjoy!"</p>
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
    if !check_cmd_args_empty(&cmd) {
        return view! {
            <InvalidOption cmd=cmd on_finished=on_finished />
        }
    }
    
    view! {
        <TypeWriter html_to_type=projects_html() callback=on_finished  />
    }
}