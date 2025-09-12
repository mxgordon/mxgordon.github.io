use std::cell::RefCell;
use std::rc::Rc;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TypewriterState {
    delay: i32,
    chunk_size: i32,
    count: Rc<RefCell<i32>>,
}

impl TypewriterState {
    pub fn new() -> Self {
        Self { delay: 4, chunk_size: 4, count: Rc::new(RefCell::new(0)) }
    }

    pub fn new_with_delay(delay: i32, ) -> Self {
        Self { delay, chunk_size:4, count: Rc::new(RefCell::new(0)) }
    }
    
    pub fn total_delay(&self) -> i32 {
        *self.count.borrow() * self.delay
    }

    pub fn text(&self, text: &str) -> Element {
        let mut count = self.count.borrow_mut();
        let starting_delay = *count * self.delay;
        let cursor_duration = text.len() as i32 * self.delay;

        // text.split_whitespace().map

        rsx! {
            {text.split_whitespace().map(|word| {
                let char_count = word.len();
                let total_delay = *count * self.delay;
                
                *count += 1;
                rsx! {
                    span {
                        class: "tw-effect",
                        style: "animation-delay: {total_delay}ms;",
                        "{word}"
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

    pub fn image_alt_loc(&self, src: &str, alt: &str) -> Element {
        let mut count = self.count.borrow_mut();
        let total_delay = *count * self.delay;
        *count += 1;

        rsx! {
            img {
                class: "tw-effect",
                src: src,
                alt: alt,
                style: "animation-delay: {total_delay}ms;",
            }
        }
    }

    pub fn image_alt(&self, src: Asset, alt: &str) -> Element {
        let mut count = self.count.borrow_mut();
        let total_delay = *count * self.delay;
        *count += 1;

        rsx! {
            img {
                class: "tw-effect",
                src: src,
                alt: alt,
                style: "animation-delay: {total_delay}ms;",
            }
        }
    }

    pub fn image(&self, src: Asset) -> Element {
        self.image_alt(src, "")
    }

    pub fn set_on_finished_callback(&self) {
        let prompt_show_delay = use_coroutine_handle::<u64>();
        info!("Total delay {}ms", self.total_delay());
        prompt_show_delay.send(self.total_delay() as u64);
    }

    pub fn t(&self, text: &str) -> Element {
        self.text(text)
    }
}