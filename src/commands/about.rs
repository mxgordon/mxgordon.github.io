use html::P;
use leptos::*;

use crate::commands::typewriter::TypeWriter;

pub fn intro_text() -> HtmlElement<P> {
    view! {
        <p>
            <p class="ascii-art">"Hi, I'm..."</p>
            <p class="orange ascii-art">" __  __               _____               _"</p>
            <p class="orange ascii-art">"|  \\/  |             / ____|             | |"</p>
            <p class="orange ascii-art">"| \\  / | __ ___  __ | |  __  ___  _ __ __| | ___  _ __  "</p>
            <p class="orange ascii-art">"| |\\/| |/ _` \\ \\/ / | | |_ |/ _ \\| '__/ _` |/ _ \\| '_ \\ "</p>
            <p class="orange ascii-art">"| |  | | (_| |>  <  | |__| | (_) | | | (_| | (_) | | | |"</p>
            <p class="orange ascii-art">"|_|  |_|\\__,_/_/\\_\\  \\_____|\\___/|_|  \\__,_|\\___/|_| |_|"</p>
            <p>"Instead of navigating with buttons and hyperlinks, on my page you will use terminal commands. Start with \"help\" for the list of commands and syntax!"</p>
            <p>"Made using "<a href="https://www.rust-lang.org/">"Rust"</a>" 🦀 and "<a href="https://leptos.dev/">"Leptos"</a>"!"</p>
        </p>
    }
}

#[component]
pub fn Intro(#[prop()] cmd: String) -> impl IntoView{
    view! {
        <TypeWriter html_to_type=intro_text() />
    }
}
