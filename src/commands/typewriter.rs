use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TypewriterState {
    delay: i32,
    count: i32,
}

impl TypewriterState {
    pub fn new() -> Self {
        Self { delay: 4, count: 0 }
    }

    pub fn new_with_delay(delay: i32) -> Self {
        Self { delay, count: 0 }
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

    pub fn set_on_finished_callback(&self) {
        let prompt_show_delay = use_coroutine_handle::<u64>();
        prompt_show_delay.send(self.total_delay() as u64);
    }

    pub fn t(&mut self, text: &str) -> Element {
        self.text(text)
    }
}