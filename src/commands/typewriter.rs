use std::cell::RefCell;
use std::rc::Rc;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum TypewriterElement {
    Text {delay: u64, chunk_size: usize, text: String, element_id: String},
    Image {element_id: String},
}

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
    
    // pub fn total_delay(&self) -> i32 {
    //     *self.count.borrow() * self.delay
    // }

    pub fn text(&self, text: &str) -> Element {
        let mut tw_seq = self.typewriter_sequence.borrow_mut();
        let element_id = format!("cmd{}-{}", self.cmd_number, tw_seq.len());

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

    pub fn image_alt_loc(&self, src: &str, alt: &str) -> Element {
        let mut tw_seq = self.typewriter_sequence.borrow_mut();
        let element_id = format!("cmd{}-{}", self.cmd_number, tw_seq.len());

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
        let element_id = format!("cmd{}-{}", self.cmd_number, tw_seq.len());

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

    pub fn finish(&self) {
        let typewriter_effect_queue = use_coroutine_handle::<Vec<TypewriterElement>>();
        typewriter_effect_queue.send(self.typewriter_sequence.take());
    }

    // pub fn set_on_finished_callback(&self) {
    //     let prompt_show_delay = use_coroutine_handle::<u64>();
    //     info!("Total delay {}ms", self.total_delay());
    //     prompt_show_delay.send(self.total_delay() as u64);
    // }

    pub fn t(&self, text: &str) -> Element {
        self.text(text)
    }
}