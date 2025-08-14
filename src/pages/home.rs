use std::ops::Deref;
use std::rc::Rc;
use dioxus::html::completions::CompleteWithBraces::template;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use crate::commands::gallery::gallery_html;
use crate::commands::intro::intro_html;

#[component]
fn Prompt() -> Element {
    rsx! {
        span {
            "user@mxgordon.com> "
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct PromptInputProps {
    prompt_input: String,
    on_submit: fn(),
    on_input: fn(Event<FormData>),
    on_keydown: fn(Event<KeyboardData>),
    autocomplete: Vec<String>,
    autocomplete_onclick: fn(&String),
}

#[allow(non_snake_case)]
pub fn PromptInput(props: PromptInputProps) -> Element {
    // let prompt_ref = create_node_ref::<Input>();

    // prompt_ref.on_load(move |e| {
    //     let _ = e.on_mount(move |e2| {
    //         e2.focus().unwrap();
    //     });
    // });
    rsx! {
        p {
            class: "prompt-line",
            Prompt {}
            input {
                r#type: "text",
                id: "prompt",
                value: props.prompt_input,
                oninput: props.on_input,
                onkeydown: props.on_keydown,
                spellcheck: "false",
                autocomplete: "off",
                aria_autocomplete: "none",
                // ref: prompt_ref,
            }
        }
    }
    
}

#[component]
pub fn Home() -> Element {
    let prompt_input = use_signal(String::new);
    let loading_stage = use_signal(|| 1);
    let mut past_cmds_html = use_signal(Vec::<Element>::new);
    let past_cmds = use_signal(|| vec!["intro".to_string()]);
    let current_past_cmd_idx = use_signal(|| -1);
    let auto_complete = use_signal(Vec::<String>::new);

    // let handleAutocompleteClick = move |cmd: &String| {
    //     writePromptInput.set(cmd.to_string());
    //     writeAutoComplete.set(search_commands(promptInput.get()).iter().map(|c| c.name.to_string()).collect());
    // };

    // let handleKeyDown = move |e: KeyboardEvent| {
    //     let key = e.key();
    //
    //     match key.as_str() {
    //         "Tab" => {
    //             e.prevent_default();
    //
    //             let new_value = promptInput.get();
    //             let potential_commands = search_commands(new_value);
    //
    //             if potential_commands.len() >= 1 {
    //                 writePromptInput.set(potential_commands[0].name.to_string());
    //             }
    //         },
    //         "ArrowUp" => {
    //             e.prevent_default();
    //             let next_idx = currentPastCmdIdx.get() + 1;
    //
    //             log!("{:?} {}", next_idx, currentPastCmdIdx.get());
    //
    //             if next_idx < pastCmds.get().len() as i32 {
    //                 log!("{:?} {} {}", next_idx, currentPastCmdIdx.get(), pastCmds.get().len());
    //                 writeCurrentPastCmdIdx.set(next_idx);
    //                 writePromptInput.set(pastCmds.get()[next_idx as usize].to_string());
    //             }
    //         },
    //         "ArrowDown" => {
    //             e.prevent_default();
    //             let next_idx = currentPastCmdIdx.get() - 1;
    //             match next_idx {
    //                 -2 => {},
    //                 -1 => {
    //                     writeCurrentPastCmdIdx.set(next_idx);
    //                     writePromptInput.set("".to_string())
    //                 },
    //                 next_idx => {
    //                     writeCurrentPastCmdIdx.set(next_idx);
    //                     writePromptInput.set(pastCmds.get()[next_idx as usize].to_string());
    //                 }
    //             }
    //         },
    //         _ => {return;}
    //     }
    //
    //     writeAutoComplete.set(search_commands(promptInput.get()).iter().map(|c| c.name.to_string()).collect());
    // };

    // let handleInput = move |e: Event| {
    //     writePromptInput.set(event_target_value(&e));
    //     let new_value = promptInput.get();
    //
    //     writeAutoComplete.set(search_commands(new_value).iter().map(|c| c.name.to_string()).collect());
    // };

    // let handleSubmit = move |e: SubmitEvent| {
    //     e.prevent_default();
    //     let input = promptInput.get();
    //     let mut cmd_splits = input.split_whitespace();
    //
    //     if let Some(cmd) = cmd_splits.next() {
    //
    //         let potential_command = get_command(cmd.to_string());
    //
    //         if let Some(command) = potential_command {
    //             writePastCmdsHtml.update(|past| {
    //                 past.push(view! {<p class="prompt-line">{make_prompt()}{promptInput.get()}</p>}.into_view());
    //                 past.push((command.function)(promptInput.get(), Box::new(move ||(writeLoadingStage.set(2)))).into_view());
    //             });
    //         } else {
    //             writePastCmdsHtml.update(|past| {
    //                 past.push(view! {<p class="prompt-line">{make_prompt()}{promptInput.get()}</p>}.into_view());
    //                 past.push(view! {<CommandNotFound cmd=promptInput.get() on_finished=Box::new(move ||(writeLoadingStage.set(2))) />}.into_view());
    //             });
    //         }
    //         writeLoadingStage.set(1);
    //
    //     } else {
    //         writePastCmdsHtml.update(|past| {
    //             past.push(view! {<p class="prompt-line">{make_prompt()}{promptInput.get()}</p>}.into_view());
    //         });
    //
    //     }
    //     writePastCmds.update(|past| {
    //         past.insert(0, promptInput.get());
    //     });
    //     writePromptInput.set("".to_string());
    //     writeAutoComplete.set(vec![]);
    //     writeCurrentPastCmdIdx.set(-1);
    // };

    let intro_cmd = rsx! {
        span {
            "intro"
        }
    }.unwrap();
    //
    // // intro_cmp
    //
    // let a = rsx!{p {class: "balls", div{id: "breh",}}}.unwrap();
    //
    // for root in a.template.roots {
    //     tracing::info!("{:?}", root);
    // }

    // past_cmds_html.set(past_cmds_html.get().unwrap())
    // past_cmds_html.push(rsx! {gallery_html {} });

    rsx! {
        div {
            id: "App",
            p {
                class: "prompt-line",
                Prompt {}
                {intro_cmd}
            }
            if *loading_stage.read() > 0 {
                // "<INTRO COMMAND>"
                intro_html {}
            }
            {past_cmds_html.read().iter()}

            if *loading_stage.read() > 1 {
                PromptInput {
                    prompt_input: "input",
                    on_submit: || tracing::info!("submitting..."),
                    on_input: |event| tracing::info!("input: {}", event.data().value()),
                    on_keydown: |event| tracing::info!("keydown: {}", event.data().key()),
                    autocomplete: vec!["testing".to_string()],
                    autocomplete_onclick: |autocomplete| tracing::info!("autocomplete: {}", autocomplete)
                }
            }

        }
    }
}
