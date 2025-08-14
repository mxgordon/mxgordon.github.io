use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use dioxus::prelude::*;
use dioxus_sdk::utils::timing::{use_interval, UseInterval};

const CURSOR_CLASS: &str = "cursor";
const HIDDEN_CLASS: &str = "hidden";

pub struct TypewriterState {
    text_signals: Vec<Option<(Signal<String>, String)>>,
    class_signals: Vec<(Signal<String>, String, String)>,
    chunk_size: usize,
    delay: u64,
}

impl TypewriterState {
    pub fn new() -> Self {
        Self {
            // texts: vec![],
            text_signals: vec![],
            class_signals: vec![],
            chunk_size: 6,
            delay: 20,
        }
    }



    pub fn text(&mut self, text: &str) -> Signal<String> {
        let signal = use_signal(|| "".to_string());

        self.text_signals.push(Some((signal, text.to_string())));

        signal
    }

    pub fn cursor(&mut self) -> Signal<String> {
        let signal = use_signal(|| "".to_string());

        self.class_signals.push((signal, "".to_string(), CURSOR_CLASS.to_string()));

        signal

    }

    pub fn cursor_with_class(&mut self, class_str: &str) -> Signal<String> {
        let signal = use_signal(|| class_str.to_string());

        self.class_signals.push((signal, class_str.to_string(), class_str.to_string() + " " + CURSOR_CLASS));

        signal

    }

    pub fn cursor_n(&mut self, count: i32) -> Signal<String> {
        let signal = use_signal(|| "".to_string());

        for _ in 0..count {
            self.class_signals.push((signal, "".to_string(), CURSOR_CLASS.to_string()));
        }

        signal

    }

    pub fn show(&mut self) -> Signal<String> {
        let signal = use_signal(|| HIDDEN_CLASS.to_string());

        self.class_signals.push((signal, HIDDEN_CLASS.to_string(), "".to_string()));
        self.text_signals.push(None);

        signal
    }

    pub fn t(&mut self, text: &str) -> Signal<String> {
        self.text(text)
    }

    pub fn c(&mut self) -> Signal<String> {
        self.cursor()
    }

    pub fn cn(&mut self, count: i32) -> Signal<String> {
        self.cursor_n(count)
    }

    pub fn cc(&mut self, class_str: &str) -> Signal<String> {
        self.cursor_with_class(class_str)
    }

    pub fn s(&mut self) -> Signal<String> {
        self.show()
    }

    pub fn typewriter(mut self) {
        let mut vec_idx = 0;
        let mut line_idx = 0;

        let interval_handler: Rc<RefCell<Option<UseInterval>>> = Rc::new(RefCell::new(None));
        let interval_handler_clone = interval_handler.clone();

        *interval_handler.borrow_mut() = Some(use_interval(Duration::from_millis(self.delay), move || {
            let next_text_signal = self.text_signals.get_mut(vec_idx);

            if let Some((text_signal)) = next_text_signal {  // if there are no more signals left (all text has been typed out)
                if let Some((signal, text)) = text_signal {
                    let (cursor_signal, from_cursor_str, to_cursor_str) = self.class_signals.get_mut(vec_idx).unwrap();
                    let next_chars = text.chars().skip(self.chunk_size * line_idx).take(self.chunk_size).collect::<String>();//.collect::<Vec<char>>();//.nth(chunk_sz * line_idx);//.char_indices().collect::<Vec<char>>().chunks(chunk_sz).nth(line_idx);

                    if !next_chars.is_empty() {
                        signal.with_mut(|s| {
                            s.push_str(&next_chars);
                        });
                        cursor_signal.replace(to_cursor_str.clone());

                        line_idx += 1;
                    } else {
                        cursor_signal.replace(from_cursor_str.clone());
                        vec_idx += 1;
                        line_idx = 0
                    }
                } else {
                    let (hide_signal, _from_hide_str, to_hide_str) = self.class_signals.get_mut(vec_idx).unwrap();
                    hide_signal.replace(to_hide_str.clone());
                    vec_idx += 1;
                    line_idx = 0;
                }
            } else {
                interval_handler_clone.borrow_mut().expect("Interval handler is None").cancel();
            }
        }));
    }
}