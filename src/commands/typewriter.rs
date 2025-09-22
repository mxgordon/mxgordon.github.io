use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum TypewriterElement {
    Text {delay: u64, chunk_size: usize, text: String, element_id: String},
    Image {element_id: String},
    End,
}

static COUNTER: AtomicUsize = AtomicUsize::new(1);
fn get_uid() -> usize { COUNTER.fetch_add(1, Ordering::Relaxed) }

#[derive(Clone, PartialEq, Debug)]
pub struct TypewriterState {
    delay: u64,
    chunk_size: usize,
    cmd_number: i32,
    typewriter_sequence: Rc<RefCell<Vec<TypewriterElement>>>
}

impl TypewriterState {
    pub fn new(cmd_number: i32) -> Self {
        Self { delay: 1, chunk_size: 3, cmd_number, typewriter_sequence: Rc::new(RefCell::new(Vec::new())) }
    }

    pub fn new_with_delay(delay: u64, chunk_size: usize, cmd_number: i32) -> Self {
        Self { delay, chunk_size, cmd_number, typewriter_sequence: Rc::new(RefCell::new(Vec::new())) }
    }

    pub fn text(&self, text: &str) -> Element {
        let mut tw_seq = self.typewriter_sequence.borrow_mut();
        let element_id = format!("cmd{}-{}", self.cmd_number, get_uid());

        tw_seq.push(TypewriterElement::Text {
            delay: self.delay,
            chunk_size: self.chunk_size,
            text: text.to_string(),
            element_id: element_id.clone(),
        });

        rsx! {
            span {
                id: element_id,
                ""
            }
        }
    }

    pub fn text_send(&self, text: &str) -> Element {
        let mut tw_seq = self.typewriter_sequence.borrow_mut();
        let element_id = format!("cmd{}-{}", self.cmd_number, get_uid());

        tw_seq.push(TypewriterElement::Text {
            delay: self.delay,
            chunk_size: self.chunk_size,
            text: text.to_string(),
            element_id: element_id.clone(),
        });

        drop(tw_seq);

        self.send();

        rsx! {
            span {
                id: element_id,
                ""
            }
        }
    }

    pub fn text_end(&self, text: &str) -> Element {
        let mut tw_seq = self.typewriter_sequence.borrow_mut();
        let element_id = format!("cmd{}-{}", self.cmd_number, get_uid());

        tw_seq.push(TypewriterElement::Text {
            delay: self.delay,
            chunk_size: self.chunk_size,
            text: text.to_string(),
            element_id: element_id.clone(),
        });

        drop(tw_seq);

        let _ = self.end();

        rsx! {
            span {
                id: element_id,
                ""
            }
        }
    }

    pub fn image_alt_loc(&self, src: &str, alt: &str) -> Element {
        let mut tw_seq = self.typewriter_sequence.borrow_mut();
        let element_id = format!("cmd{}-{}", self.cmd_number, get_uid());

        tw_seq.push(TypewriterElement::Image {element_id: element_id.clone()});

        rsx! {
            img {
                id: element_id,
                class: "hidden",
                src: src,
                alt: alt,
            }
        }
    }

    pub fn image_alt(&self, src: Asset, alt: &str) -> Element {
        let mut tw_seq = self.typewriter_sequence.borrow_mut();
        let element_id = format!("cmd{}-{}", self.cmd_number, get_uid());

        tw_seq.push(TypewriterElement::Image {element_id: element_id.clone()});

        rsx! {
            img {
                id: element_id,
                class: "hidden",
                src: src,
                alt: alt,
            }
        }
    }

    pub fn image(&self, src: Asset) -> Element {
        self.image_alt(src, "")
    }

    pub fn end(&self) -> Element {
        let mut tw_seq = self.typewriter_sequence.borrow_mut();
        tw_seq.push(TypewriterElement::End);
        drop(tw_seq);

        self.send();
        
        rsx!()
    }

    pub fn send(&self)  {
        let typewriter_effect_queue = use_coroutine_handle::<Vec<TypewriterElement>>();

        typewriter_effect_queue.send(self.typewriter_sequence.take());
    }

    pub fn t(&self, text: &str) -> Element {
        self.text(text)
    }
    pub fn ts(&self, text: &str) -> Element {
        self.text_send(text)
    }
    pub fn te(&self, text: &str) -> Element {
        self.text_end(text)
    }
}