use ev::Event;
use ev::SubmitEvent;
use html::span;
use html::AnyElement;
use html::Input;
use html::P;
use leptos::*;
use leptos::logging::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::commands::about::*;
use crate::commands::search::*;
use crate::commands::typewriter::*;

fn autofocus(e: Event) {
    log!("e{:?}", e);
}

fn make_prompt() -> HtmlElement<html::Span>{
    view! {
        <span>"user@mxgordon.com> "</span>
    }
}

#[component]
pub fn PromptInput(
    #[prop()] prompt_input: ReadSignal<String>,
    #[prop()] on_submit: Box<dyn Fn(SubmitEvent) + 'static>,
    #[prop()] on_input: Box<dyn Fn(Event) + 'static>,
) -> impl IntoView {
    let prompt_ref = create_node_ref::<Input>();

    prompt_ref.on_load(move |e| {
        let _ = e.on_mount(move |e2| {
            e2.focus().unwrap();
        });
    });

    view! {
        <p class="prompt-line" >{make_prompt()}
            <form on:submit=on_submit>
                <input ref=prompt_ref type="text" id="prompt" prop:value=prompt_input on:input=on_input  />
            </form>
        </p>
    }
    
}

#[component]
pub fn Home() -> impl IntoView {
    let (promptInput, writePromptInput) = create_signal("".to_string());
    let (loadingStage, writeLoadingStage) = create_signal(0);
    let (pastCmds, writePastCmds) = create_signal::<Vec<View>>(vec![]);

    let handleInput = move |e: Event| {
        writePromptInput.set(event_target_value(&e));
    };

    let handleSubmit = move |e: SubmitEvent| {
        e.prevent_default();
        let potential_commands = search_commands(promptInput.get());

        if potential_commands.len() == 0 {
            writePastCmds.update(|past| {
                past.push(view! {<p class="prompt-line">{make_prompt()}{promptInput.get()}</p>}.into_view());
                past.push(view! {<CommandNotFound cmd=promptInput.get() on_finished=Box::new(move ||(writeLoadingStage.set(2))) />}.into_view());
            });
        } else {
            writePastCmds.update(|past| {
                past.push(view! {<p class="prompt-line">{make_prompt()}{promptInput.get()}</p>}.into_view());
                past.push((potential_commands[0].function)(promptInput.get(), Box::new(move ||(writeLoadingStage.set(2)))).into_view());
            });
        }
        writeLoadingStage.set(1);
        writePromptInput.set("".to_string());
    };

    let s = view!{<span>"intro"</span>};

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
                <p class="prompt-line">{make_prompt()}
                <TypeWriter html_to_type=s base_element=span() delay=200 chunk_sz=1 callback=Box::new(move ||(writeLoadingStage.set(1))) /></p>

                <Show when=move || (loadingStage.get() > 0)>
                    <TypeWriter html_to_type=intro_text() callback=Box::new(move ||(writeLoadingStage.set(2))) />
                </Show>

                {pastCmds}

                <Show when=move || (loadingStage.get() > 1)>
                    <PromptInput prompt_input=promptInput on_submit=Box::new(handleSubmit) on_input=Box::new(handleInput) />
                </Show>
            </div>
        </ErrorBoundary>
    }
}
