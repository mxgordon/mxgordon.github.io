use html::P;
use leptos::*;

use crate::commands::{typewriter::TypeWriter, utils::*};

fn intro_html() -> HtmlElement<P> {
    view! {
        <p>
            <p class="ascii-art">"Hi, I'm..."</p>
            <p class="orange ascii-art">" __  __               _____               _"</p>
            <p class="orange ascii-art">"|  \\/  |             / ____|             | |"</p>
            <p class="orange ascii-art">"| \\  / | __ ___  __ | |  __  ___  _ __ __| | ___  _ __  "</p>
            <p class="orange ascii-art">"| |\\/| |/ _` \\ \\/ / | | |_ |/ _ \\| '__/ _` |/ _ \\| '_ \\ "</p>
            <p class="orange ascii-art">"| |  | | (_| |>  <  | |__| | (_) | | | (_| | (_) | | | |"</p>
            <p class="orange ascii-art">"|_|  |_|\\__,_/_/\\_\\  \\_____|\\___/|_|  \\__,_|\\___/|_| |_|"</p>
            <p>"Instead of navigating with buttons and hyperlinks, on my page you will use terminal commands. Start with `"<span class="orange">"help"</span>"` for the list of commands and syntax!"</p>
            <p>"Made using "<a href="https://www.rust-lang.org/" target="_blank" rel="noopener noreferrer">"Rust"</a>" 🦀 and "<a href="https://leptos.dev/" target="_blank" rel="noopener noreferrer">"Leptos"</a>"!"</p>
        </p>
    }
}

#[component]
pub fn Intro(#[prop()] cmd: String, #[prop(default=Box::new(|| ()))] on_finished: Box<dyn Fn() + 'static>) -> impl IntoView{
    if !check_cmd_args_empty(&cmd) {
        return view! {
            <InvalidOption cmd=cmd on_finished=on_finished />
        }
    }

    view! {
        <TypeWriter html_to_type=intro_html() callback=on_finished />
    }
}

fn about_html() -> HtmlElement<P> {
    view! {
        <p>
            <h2>"About Me"</h2>
            <p>"I am a computer science BS/MS student at Worcester Polytechnic University ("<a href="https://www.wpi.edu/" target="_blank" rel="noopener noreferrer">"WPI"</a>"). I have a strong interest in systems-level programming and financial technology. In high school, I was on a FIRST robotics ("<a href="https://www.firstinspires.org/robotics/frc" target="_blank" rel="noopener noreferrer">"FRC"</a>") team, where I discovered my passions for programming, electronics, and additive manufactoring."</p>
            <p>"Last summer, I interned at "<a href="https://www.finra.org/about" target="_blank" rel="noopener noreferrer">"FINRA"</a>" where I explored my interest for financial technology. Fall semester this year (2024), I spent 2 months studying abroad in Panama City, working with "<a href="https://www.simpleaf.earth/" target="_blank" rel="noopener noreferrer">"simple af"</a>". A growing sustainable design studio manufacturing fabric goods from used fabrics like billboards and clothes, expanding the circular economy in Latin America."</p>
            <p>"In my freetime I like to shoot film photography (try the `"<span class="orange">"gallery"</span>"` command). Additionally, I enjoy 3D printing and tinkering with my printer."</p>
            <p>"For more information about me, check out these links:"</p>

            <div class="about-links">
                <a href="https://github.com/mxgordon" target="_blank" rel="noopener noreferrer"><p><img src="/icons/github.png" />GitHub</p></a>
                <a href="https://www.linkedin.com/in/max-gordon-533423221/" target="_blank" rel="noopener noreferrer"><p><img src="/icons/linkedin.png" />LinkedIn</p></a>
                <a href="/documents/Max_Gordon_resume.pdf" target="_blank" rel="noopener noreferrer"><p><img src="/icons/resume.png" />Resume</p></a>
            </div>
            
        </p>
    }
}

#[component]
pub fn About(#[prop()] cmd: String, #[prop(default=Box::new(|| ()))] on_finished: Box<dyn Fn() + 'static>) -> impl IntoView{
    if !check_cmd_args_empty(&cmd) {
        return view! {
            <InvalidOption cmd=cmd on_finished=on_finished />
        }
    }

    view! {
        <TypeWriter html_to_type=about_html() callback=on_finished />
    }
}
