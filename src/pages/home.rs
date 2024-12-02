use std::ops::Deref;
use std::rc::Rc;

use ev::Event;
use ev::KeyboardEvent;
use ev::SubmitEvent;
use html::span;
use html::AnyElement;
use html::Input;
use leptos::*;
use leptos::logging::log;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, Element};

use crate::commands::about::*;
use crate::commands::search::*;
use crate::commands::typewriter::*;
use crate::commands::utils::*;

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
    #[prop()] on_keydown: Box<dyn Fn(KeyboardEvent) + 'static>,
    #[prop()] autocomplete: ReadSignal<Vec<String>>,
    #[prop()] autocomplete_onclick: Rc<Box<dyn Fn(&String) + 'static>>,
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
                <input ref=prompt_ref type="text" id="prompt" prop:value=prompt_input on:input=on_input on:keydown=on_keydown spellcheck="false" autocomplete="off" aria-autocomplete="none" />

                <div class="autocomplete-options">
                    <For each=move || autocomplete.get() key=|cmd_str| cmd_str.clone() children=move |cmd| {
                        let autocomplete_onclick = Rc::clone(&autocomplete_onclick);
                        let cmd_clone = cmd.clone();
                        view!{
                            <p on:click=move |_e| {
                                autocomplete_onclick(&cmd_clone);
                            }>{cmd}</p>
                        }
                    } />
                </div>
            </form>
        </p>
    }
    
}

#[component]
pub fn Home() -> impl IntoView {
    let (promptInput, writePromptInput) = create_signal("".to_string());
    let (loadingStage, writeLoadingStage) = create_signal(0);
    let (pastCmdsHtml, writePastCmdsHtml) = create_signal::<Vec<View>>(vec![]);
    let (pastCmds, writePastCmds) = create_signal::<Vec<String>>(vec!["intro".to_string()]);
    let (currentPastCmdIdx, writeCurrentPastCmdIdx) = create_signal(-1);
    let (autocomplete, writeAutoComplete) = create_signal::<Vec<String>>(vec![]);

    let handleAutocompleteClick = move |cmd: &String| {
        writePromptInput.set(cmd.to_string());
        writeAutoComplete.set(search_commands(promptInput.get()).iter().map(|c| c.name.to_string()).collect());
    };

    let handleKeyDown = move |e: KeyboardEvent| {
        let key = e.key();

        match key.as_str() {
            "Tab" => {
                e.prevent_default();

                let new_value = promptInput.get();
                let potential_commands = search_commands(new_value);

                if potential_commands.len() >= 1 {
                    writePromptInput.set(potential_commands[0].name.to_string());
                }
            },
            "ArrowUp" => {
                e.prevent_default();
                let next_idx = currentPastCmdIdx.get() + 1;

                log!("{:?} {}", next_idx, currentPastCmdIdx.get());
                
                if next_idx < pastCmds.get().len() as i32 {
                    log!("{:?} {} {}", next_idx, currentPastCmdIdx.get(), pastCmds.get().len());
                    writeCurrentPastCmdIdx.set(next_idx);
                    writePromptInput.set(pastCmds.get()[next_idx as usize].to_string());
                }
            },
            "ArrowDown" => {
                e.prevent_default();
                let next_idx = currentPastCmdIdx.get() - 1;
                match next_idx {
                    -2 => {},
                    -1 => {
                        writeCurrentPastCmdIdx.set(next_idx);
                        writePromptInput.set("".to_string())
                    },
                    next_idx => {
                        writeCurrentPastCmdIdx.set(next_idx);
                        writePromptInput.set(pastCmds.get()[next_idx as usize].to_string());
                    }
                }
            },
            _ => {return;}
        }
        
        writeAutoComplete.set(search_commands(promptInput.get()).iter().map(|c| c.name.to_string()).collect());
    };

    let handleInput = move |e: Event| {
        writePromptInput.set(event_target_value(&e));
        let new_value = promptInput.get();

        writeAutoComplete.set(search_commands(new_value).iter().map(|c| c.name.to_string()).collect());
    };

    let handleSubmit = move |e: SubmitEvent| {
        e.prevent_default();

        let potential_command = get_command(promptInput.get());
        
        if let Some(command) = potential_command {
            writePastCmdsHtml.update(|past| {
                past.push(view! {<p class="prompt-line">{make_prompt()}{promptInput.get()}</p>}.into_view());
                past.push((command.function)(promptInput.get(), Box::new(move ||(writeLoadingStage.set(2)))).into_view());
            });
        } else {
            writePastCmdsHtml.update(|past| {
                past.push(view! {<p class="prompt-line">{make_prompt()}{promptInput.get()}</p>}.into_view());
                past.push(view! {<CommandNotFound cmd=promptInput.get() on_finished=Box::new(move ||(writeLoadingStage.set(2))) />}.into_view());
            });
        }
        writePastCmds.update(|past| {
            past.insert(0, promptInput.get());
        });
        writeLoadingStage.set(1);
        writePromptInput.set("".to_string());
        writeAutoComplete.set(vec![]);
        writeCurrentPastCmdIdx.set(-1);
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
                    <Intro cmd="intro".to_string() on_finished=Box::new(move ||(writeLoadingStage.set(2))) />
                </Show>

                {move || {log!("{:?}", pastCmdsHtml.get()); pastCmdsHtml}}
                // {move || {pastCmdsHtml.get()}}
                // {pastCmdsHtml}

                <Show when=move || (loadingStage.get() > 1)>
                    <PromptInput prompt_input=promptInput on_submit=Box::new(handleSubmit) on_input=Box::new(handleInput) on_keydown=Box::new(handleKeyDown) autocomplete=autocomplete autocomplete_onclick=Rc::new(Box::new(handleAutocompleteClick)) />
                </Show>
            </div>
        </ErrorBoundary>
    }
}
