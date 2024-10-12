use ev::Event;
use html::span;
use leptos::*;
use leptos::logging::log;
use wasm_bindgen::JsCast;

use crate::commands::about::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let (_promptInput, setPromptInput) = create_signal("".to_string());

    let handleChange = move |e: Event| {
        // Get the target element and cast it to HtmlElement
        if let Some(target) = e.target() {
            if let Some(p_element) = target.dyn_ref::<web_sys::HtmlElement>() {
                // Now you can access the text content of the p element
                log!("{}", p_element.text_content().unwrap_or_default());
                setPromptInput.set(p_element.text_content().unwrap_or_default());
            }
        } else {
            log!("Event target is not an HtmlElement");
        }
    };

    let s = view!{<span>"into"</span>};

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>
                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            }
        }>

            <div id="App">
                // <p>"Hi, I'm "<span class="orange">"Max Gordon"</span>" and this is my personal website. Please explore it!"</p>
                <p>"user@mxgordon.com>"
                <TypeWriter html_to_type=s base_element=span() /></p>

                // <Intro2 />
                // <Intro2 />
                <TypeWriter html_to_type=intro_text() />
                // <p>"user@mxgordon.com>"<input id="term-inp" autofocus on:change=handleChange></input><span id="cursor" /></p>
                <p>"user@mxgordon.com>"<p contenteditable autofocus on:input=handleChange id="input"></p></p>
            </div>
        </ErrorBoundary>
    }
}
