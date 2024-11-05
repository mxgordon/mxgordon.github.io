use std::rc::Rc;
use std::time::Duration;
use std::{cell::RefCell, fmt::Debug};

use html::{AnyElement, ToHtmlElement, a, div, p, span};
use leptos::*;

use leptos::logging::log;
use leptos_dom::helpers::IntervalHandle;
use log::debug;
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};

#[derive(Clone)]
enum TypeElement {
    Text { t: String },
    Element { e: HtmlElement<AnyElement> },
    EndElement(),
    StartText(),
    EndText(),
    Comment{ c: Node},
}

impl Debug for TypeElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeElement::Text { t } => f.write_str(&format!("Text: {}", t)),
            TypeElement::Element { e } => f.write_str(&format!("Element: {}", e.node_name())),
            TypeElement::EndElement() => f.write_str("EndElement"),
            TypeElement::StartText() => f.write_str("StartText"),
            TypeElement::EndText() => f.write_str("EndText"),
            TypeElement::Comment{c} => f.write_str(&format!("Comment: {}", c.text_content().unwrap_or_default())),
        }
    }
}

fn break_down_html<'a>(
    html: &HtmlElement<AnyElement>,
    seq: &'a mut Vec<TypeElement>,
    chunk_sz: usize
) -> &'a mut Vec<TypeElement> {
    break_down_node(&html.get_root_node(), seq, chunk_sz)
}

fn break_down_node<'a>(root: &Node, seq: &'a mut Vec<TypeElement>, chunk_sz: usize) -> &'a mut Vec<TypeElement> {
    let root_name = root.node_name();

    if root.node_type() == 3 {
        seq.push(TypeElement::StartText());

        let text = root.text_content().unwrap();
        let mut chars = text.chars().peekable();

        while chars.peek().is_some() {
            let s: String = chars.clone().take(chunk_sz).collect();
            for _ in 0..chunk_sz {
                chars.next();
            }

            seq.push(TypeElement::Text { t: s });
        }

        seq.push(TypeElement::EndText());

        return seq;
    } else if root.node_type() == 8 { // Copy comments as well (generally generated by Leptos)
        seq.push(TypeElement::Comment{c: root.clone()});
        return seq;
    }

    let root_element: Element = root.clone().dyn_into().unwrap();

    let attrs = root_element.get_attribute_names();

    let attrs: Vec<(&'static str, Attribute)> = attrs
        .iter()
        .map(|attr| {
            let name = attr.as_string().unwrap().to_owned();
            let name_str: &'static str = Box::leak(name.into_boxed_str());

            let value = root_element.get_attribute(&name_str).unwrap();
            (name_str, Attribute::String(value.into()))
        })
        .collect();

    let mut next_element: Option<HtmlElement<AnyElement>> = None;

    match root_name.as_str() {
        "P" => {
            next_element = Some(p().into());
        }
        "SPAN" => {
            next_element = Some(span().into());
        }
        "A" => {
            next_element = Some(a().into());
        }
        other => {
            log!("Unknown element: {}", other);
        }
    }

    let _ = next_element
        .clone()
        .expect("Element should be initialized")
        .attrs(attrs);

    seq.push(TypeElement::Element {
        e: next_element.expect("Element should be initialized").into(),
    });

    for i in 0..root.child_nodes().length() {
        let child = root.child_nodes().get(i).unwrap();
        break_down_node(&child, seq, chunk_sz);
    }

    seq.push(TypeElement::EndElement());

    seq
}


#[component]
pub fn TypeWriter(
    #[prop(into)] html_to_type: HtmlElement<AnyElement>,
    #[prop(default=div().into(), into)] base_element: HtmlElement<AnyElement>,
    #[prop(default=20)] delay: u64,
    #[prop(default=Box::new(|| ()))] callback: Box<dyn Fn() + 'static>,
    #[prop(default=6)] chunk_sz: usize,
) -> impl IntoView {
    let container_div_ref = create_node_ref::<AnyElement>();

    let mut charSeq: Vec<TypeElement> = Vec::new();

    break_down_html(&html_to_type.clone(), &mut charSeq, chunk_sz);

    container_div_ref.on_load(move |e| {
        let _ = e.on_mount(move |e| {
            let idxRef = RefCell::new(0);
            let intervalHandleRef: Rc<RefCell<Option<IntervalHandle>>> =
                Rc::new(RefCell::new(None));

            let current_element: RefCell<HtmlElement<AnyElement>> = RefCell::new(e.into());
            let current_text: RefCell<Option<Node>> = RefCell::new(None);


            let cb = {
                let intervalHandleRef = intervalHandleRef.clone();
                move || {
                    let mut current_element = current_element.borrow_mut();
                    let mut idx = idxRef.borrow_mut();
                    let mut current_text = current_text.borrow_mut();

                    let mut iter_again = true;

                    while iter_again {
                        if *idx >= charSeq.len() {

                            intervalHandleRef.borrow_mut().unwrap().clear();
                            callback();
                            return;
                        }

                        let next = &charSeq[*idx];

                        match next {
                            TypeElement::EndElement() => {
                                let classes = current_element.clone().class_name();
                                let _ = current_element.clone().attr("class", classes.replace("typing", ""));

                                let parent_node = current_element.parent_element().unwrap();
                                *current_element = ToHtmlElement::to_leptos_element(&parent_node);
                            }
                            TypeElement::Text { t } => {
                                let mut text = current_text
                                    .as_ref()
                                    .unwrap()
                                    .text_content()
                                    .unwrap_or_default();

                                text.push_str(&t);

                                current_text.as_ref().unwrap().set_text_content(Some(&text));
                                
                                let window = window();
                                let document = document();
                                let body = document.body().unwrap();
                                let current_scroll = window.scroll_y().unwrap();
                                let target_scroll = body.scroll_height() as f64 - window.inner_height().unwrap().as_f64().unwrap();
                            
                                if (current_scroll + 1.) < target_scroll {  // +1 to avoid floating point errors
                                    log!("scrolling... {} {}", current_scroll, target_scroll);
                                    window.scroll_to_with_x_and_y(0.0, target_scroll);
                                }

                                iter_again = false;
                            }
                            TypeElement::Element { e } => {
                                let classes = current_element.clone().class_name();
                                let _ = current_element.clone().attr("class", classes.replace("typing", ""));

                                let _ = current_element.append_child(&e);
                                *current_element = e.clone();
                                
                                let _ = current_element.clone().classes("typing");
                            }
                            TypeElement::StartText() => {
                                let _ = current_element.append_with_str_1("");

                                *current_text = Some(current_element.last_child().unwrap()); // unwrapping then Some, so it'll panic if something weird happens
                            }
                            TypeElement::EndText() => {
                                *current_text = None;
                            }
                            TypeElement::Comment{c} => {
                                let _ = current_element.append_child(&c);
                            }
                        }

                        *idx += 1;
                    }
                }
            };
            *intervalHandleRef.borrow_mut() =
                set_interval_with_handle(cb, Duration::from_millis(delay)).ok();
        });
    });

    base_element.node_ref(container_div_ref)
}
