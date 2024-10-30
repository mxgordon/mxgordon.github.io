use ev::Event;
use ev::KeyboardEvent;
use ev::SubmitEvent;
use html::span;
use leptos::*;
use leptos::logging::log;
use wasm_bindgen::JsCast;

use crate::commands::about::*;
use crate::commands::search::*;
use crate::commands::typewriter::*;

fn autofocus(e: Event) {
    log!("e{:?}", e);
}

#[component]
pub fn Home() -> impl IntoView {
    let (promptInput, writePromptInput) = create_signal("".to_string());
    let (loadingStage, writeLoadingStage) = create_signal(0);
    let (pastCmds, writePastCmds) = create_signal::<Vec<View>>(vec![]);

    let prompt = "user@mxgordon.com> ";

    let handleInput = move |e: Event| {
        writePromptInput.set(event_target_value(&e));
    };

    let handleSubmit = move |e: SubmitEvent| {
        e.prevent_default();
        // log!("{:?}", search_commands(promptInput.get()));
        let potential_commands = search_commands(promptInput.get());

        if potential_commands.len() == 0 {
            writePastCmds.update(|past| {
                past.push(view! {<p>{prompt}{promptInput.get()}</p>}.into_view());
                past.push(view! {<CommandNotFound cmd=promptInput.get() />}.into_view());

                log!("{:?}", promptInput.get());

                writePromptInput.set("".to_string());

                log!("{:?}", promptInput.get());
            });
            // writePastCmds.set(vec![view! { <CommandNotFound cmd=promptInput.get() /> }]);
        }
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
                <p class="prompt-line">{prompt}
                <TypeWriter html_to_type=s base_element=span() delay=150 chunk_sz=1 callback=Box::new(move ||(writeLoadingStage.set(1))) /></p>

                <Show when=move || (loadingStage.get() > 0)>
                    <TypeWriter html_to_type=intro_text() delay=20 chunk_sz=3 callback=Box::new(move ||(writeLoadingStage.set(2))) />
                </Show>

                {pastCmds}

                <Show when=move || (loadingStage.get() > 1)>
                    // <p class="prompt-line">{prompt}<p contenteditable autofocus on:input=handleChange id="input"></p></p>
                    <p class="prompt-line" >{prompt}
                        <form on:submit=handleSubmit >
                            <input name="thing" type="text" prop:value=promptInput on:input=handleInput autofocus />
                        </form>
                    </p>
                </Show>
            </div>
        </ErrorBoundary>
    }
}
