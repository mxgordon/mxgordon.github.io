use std::time::{Duration};
use async_std::prelude::StreamExt;
use async_std::task;
use chrono::Local;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use crate::commands::intro::Intro;
use crate::commands::typewriter::{TypewriterElement, TypewriterState};
use crate::commands::utils::{get_command, search_commands, CommandNotFound, CommandProps};
use itertools::Itertools;

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
    on_submit: Callback<Event<FormData>>,
    on_input: Callback<Event<FormData>>,
    on_keydown: Callback<Event<KeyboardData>>,
    auto_complete: Signal<Vec<String>>,
    auto_complete_onclick: Callback<String>,
}

#[allow(non_snake_case)]
pub fn PromptInput(props: PromptInputProps) -> Element {
    rsx! {
        p {
            class: "prompt-line",
            Prompt {}
            form {
                onsubmit: props.on_submit,
                input {
                    r#type: "text",
                    id: "prompt",
                    value: props.prompt_input,
                    oninput: props.on_input,
                    onkeydown: props.on_keydown,
                    spellcheck: "false",
                    autocomplete: "off",
                    aria_autocomplete: "none",
                    onmounted: |e| async move {e.set_focus(true).await.expect("TODO: panic message");},
                }

                div {
                    class: "autocomplete-options",
                    for auto_complete_option in props.auto_complete.read().clone() {
                        p {
                            onclick: move |_| {props.auto_complete_onclick.call(auto_complete_option.clone())},
                            {auto_complete_option.clone()}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn IntroCommand() -> Element {
    let t = TypewriterState::new_with_delay(200, 1,-1);

    let rtn = rsx! {
        span {
            {t.te("intro")}
        }
    };

    // t.finish();

    rtn
}

#[component]
pub fn Home() -> Element {
    let mut prompt_input = use_signal(String::new);
    let mut loading_stage = use_signal(|| 0);
    let mut past_cmds_html = use_signal(Vec::<Element>::new);
    let mut past_cmds = use_signal(|| vec!["intro".to_string()]);
    let mut current_past_cmd_idx = use_signal(|| -1);
    let mut auto_complete = use_signal(Vec::<String>::new);

    let handle_autocomplete_click = move |cmd: String| {
        prompt_input.set(cmd.to_string());
        auto_complete.set(search_commands(&prompt_input.peek()).iter().map(|c| c.name.to_string()).collect());
    };

    let handle_key_down = move |e: Event<KeyboardData>| {
        match e.key() {
            Key::Tab => {
                e.prevent_default();

                let new_value = prompt_input.peek().clone();
                let potential_commands = search_commands(&new_value);

                if !potential_commands.is_empty() {
                    prompt_input.set(potential_commands[0].name.to_string());
                }
            },
            Key::ArrowUp => {
                e.prevent_default();
                let next_idx = *current_past_cmd_idx.peek() + 1;

                info!("{:?} {}", next_idx, current_past_cmd_idx.peek());

                if next_idx < past_cmds.len() as i32 {
                    info!("{:?} {} {}", next_idx, current_past_cmd_idx.peek(), past_cmds.len());
                    current_past_cmd_idx.set(next_idx);
                    prompt_input.set(past_cmds.get(next_idx as usize).expect("Index out of bounds").to_string());
                }
            },
            Key::ArrowDown => {
                e.prevent_default();
                let next_idx = *current_past_cmd_idx.peek() - 1;
                match next_idx {
                    -2 => {},
                    -1 => {
                        current_past_cmd_idx.set(next_idx);
                        prompt_input.set("".to_string())
                    },
                    next_idx => {
                        current_past_cmd_idx.set(next_idx);
                        prompt_input.set(past_cmds.get(next_idx as usize).expect("Index out of range").to_string());
                    }
                }
            },
            _ => {return;}
        }

        auto_complete.set(search_commands(&prompt_input.peek()).iter().map(|c| c.name.to_string()).collect());
    };

    let handle_input = move |e: Event<FormData>| {
        prompt_input.set(e.value());
        let new_value = prompt_input.peek();

        auto_complete.set(search_commands(&new_value).iter().map(|c| c.name.to_string()).collect());
    };

    let handle_submit = move |e: Event<FormData>| {
        e.prevent_default();
        let input = prompt_input.peek().clone();
        let mut cmd_splits = input.split_whitespace();

        if let Some(cmd) = cmd_splits.next() {

            let potential_command = get_command(cmd);

            if let Some(command) = potential_command {
                past_cmds_html.with_mut(|past| {
                    past.push(rsx! {p { class: "prompt-line", Prompt {} {prompt_input.peek().clone()}}});
                    past.push(
                        (command.function)(CommandProps::new(prompt_input.peek().clone(), past_cmds.peek().len() as i32)))
                });
            } else {
                past_cmds_html.with_mut(|past| {
                    past.push(rsx! {p { class: "prompt-line", Prompt {} {prompt_input.peek().clone()}}});
                    past.push(rsx! {
                        CommandNotFound {cmd: prompt_input.peek().clone(), typewriter_state: TypewriterState::new(past_cmds.peek().len() as i32)}

                    });
                });
            }
            loading_stage.set(1);

        } else {
            past_cmds_html.with_mut(|past| {
                past.push(rsx! {p { class: "prompt-line", Prompt {} {prompt_input.peek().clone()}}});
            });

        }
        past_cmds.with_mut(|past| {
            past.insert(0, prompt_input.peek().clone());
        });
        prompt_input.set("".to_string());
        auto_complete.set(vec![]);
        current_past_cmd_idx.set(-1);
    };

    let _prompt_show_delay = use_coroutine(move |mut rx: UnboundedReceiver<Vec<TypewriterElement>>| async move {
        while let Some(typewriter_elements) = rx.next().await {
            info!("Received elements {:?}", typewriter_elements);
            task::sleep(Duration::from_millis(1)).await;

            for type_ele in typewriter_elements {
                match type_ele {
                    TypewriterElement::Text { delay, chunk_size, text, element_id } => {
                        document::eval(&format!(r#"
                        window["element"] = document.getElementById("{}");
                        window["element"].classList.add("cursor-end");"#, element_id));

                        for chunk in text.chars().chunks(chunk_size).into_iter() {
                            document::eval(&format!(r#"window["element"].textContent += {:?};"#, chunk.collect::<String>()));

                            task::sleep(Duration::from_millis(delay)).await;
                        }
                        document::eval(r#"window["element"].classList.remove("cursor-end");"#);
                    }
                    TypewriterElement::Image { element_id } => {
                        document::eval(&format!(r#"
                        window["element"] = document.getElementById("{}");
                        window["element"].classList.remove("hidden");"#, element_id));
                    }
                    TypewriterElement::End => {
                        loading_stage.with_mut(|stage| *stage += 1);
                    }
                }
            }
        }
    });
            rsx! {
        div {
            id: "App",
            p {
                class: "prompt-line",
                Prompt {}
                IntroCommand {}
            }
            if *loading_stage.read() > 0 {
                Intro { cmd: "intro", typewriter_state: TypewriterState::new(0)}
            }
            {past_cmds_html.read().iter()}

            if *loading_stage.read() > 1 {
                PromptInput {
                    prompt_input: {prompt_input},
                    on_submit:  handle_submit,
                    on_input: handle_input,
                    on_keydown: handle_key_down,
                    auto_complete: auto_complete,
                    auto_complete_onclick: handle_autocomplete_click,
                }
            }
        }
    }
}
