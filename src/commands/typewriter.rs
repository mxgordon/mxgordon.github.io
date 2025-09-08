use std::time::Duration;
use async_std::task;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TypewriterState {
    delay: i32,
    count: i32,
    typewriter_effect: bool,
}

impl TypewriterState {
    pub fn new() -> Self {
        Self { delay: 4, count: 0, typewriter_effect: true }
    }

    pub fn new_no_typewriter_effect() -> Self {
        Self { delay: 0, count: 0, typewriter_effect: false }
    }

    pub fn new_with_delay(delay: i32) -> Self {
        Self { delay, count: 0, typewriter_effect: true }
    }
    
    pub fn total_delay(&self) -> i32 {
        self.count * self.delay
    }

    pub fn text(&mut self, text: &str) -> Element {
        let starting_delay = self.count * self.delay;
        let cursor_duration = text.len() as i32 * self.delay;

        rsx! {
            {text.chars().map(|letter| {
                let total_delay = self.count * self.delay;
                self.count += 1;
                rsx! {
                    span {
                        class: "tw-effect",
                        style: "animation-delay: {total_delay}ms;",
                        "{letter}"
                    }
                }
            })}
            span {
                class: "cursor",
                style: "animation-delay: {starting_delay}ms; animation-duration: {cursor_duration}ms;",
                ""
            }
        }
    }



    // pub fn on_finished(&self) {
    //     let prompt_show_delay = use_coroutine_handle::<u64>();
    //     prompt_show_delay.send(self.total_delay() as u64);
    // }

    pub fn set_on_finished_callback(&self) {
        let prompt_show_delay = use_coroutine_handle::<u64>();
        prompt_show_delay.send(self.total_delay() as u64);
        // let total_delay = self.total_delay() as u64;

        // spawn
        // spawn(async move {
        //     info!("Spawn delay...");
        //     task::sleep(Duration::from_millis(total_delay)).await;
        //     info!("Spawn delay over");
        //     on_finished.call(());
        //     info!("on_finished called");
        // });

        // let _ = use_coroutine(move |_rx: UnboundedReceiver<()>| async move {
        //     info!("Starting delay...");
        //     task::sleep(Duration::from_millis(total_delay)).await;
        //     info!("Delay over");
        //     on_finished.call(());
        //     info!("on_finished called");
        // });
    }

    pub fn t(&mut self, text: &str) -> Element {
        self.text(text)
    }
}