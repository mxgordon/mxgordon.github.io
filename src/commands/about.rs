use std::{cell::RefCell, fmt::Debug};
use std::rc::Rc;
use std::time::Duration;

use html::{a, div, p, span, AnyElement, ToHtmlElement, P};
use leptos::*;

use leptos::logging::log;
use leptos_dom::helpers::IntervalHandle;
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};


#[derive(Clone)]
enum TypeElement {
    Character {c: char},
    Element {e: HtmlElement<AnyElement>},
    EndElement(),
    StartText(),
    EndText(),
}

impl Debug for TypeElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeElement::Character{c} => {
                f.write_str(&format!("Character: {}", c))
            }
            TypeElement::Element{e} => {
                f.write_str(&format!("Element: {}", e.node_name()))
            }
            TypeElement::EndElement() => {
                f.write_str("EndElement")
            }
            TypeElement::StartText() => {
                f.write_str("StartText")
            }
            TypeElement::EndText() => {
                f.write_str("EndText")
            }
        }
    }
}


fn break_down_html<'a>(html: &HtmlElement<AnyElement>, seq: &'a mut Vec<TypeElement>) -> &'a mut Vec<TypeElement> {
    break_down_node(&html.get_root_node(), seq)
}

fn break_down_node<'a>(root: &Node, seq: &'a mut Vec<TypeElement>) -> &'a mut Vec<TypeElement> {
    log!("{:?} {} {:?}", root, root.node_type(), root.text_content()); 

    let root_name = root.node_name();

    if root.node_type() == 3 {
        seq.push(TypeElement::StartText());

        let text = root.text_content().unwrap();
        for c in text.chars() {
            seq.push(TypeElement::Character{c});
        }

        seq.push(TypeElement::EndText());

        return seq;
    }
    
    let root_element: Element = root.clone().dyn_into().unwrap();
    let attrs = root_element.get_attribute_names();
    // let attrIterator = attrs.iter();
    let attrs: Vec<(&'static str, Attribute)> = attrs.iter().map(|attr| {
            let name = attr.as_string().unwrap().to_owned();
            let name_str: &'static str = Box::leak(name.into_boxed_str());

            let value = root_element.get_attribute(&name_str).unwrap();
            (name_str, Attribute::String(value.into()))
        }).collect();

    let mut next_element: Option<HtmlElement<AnyElement>> = None;
    
    match root_name.as_str() {
        "P"=>{
            next_element = Some(p().into());
        }
        "SPAN"=>{
            next_element = Some(span().into());
        }
        "A"=>{
            next_element = Some(a().into());
        }
        other=>{
            log!("Unknown element: {}", other);
        }
    }
    
    let _ = next_element.clone().expect("Element should be initialized").attrs(attrs);

    seq.push(TypeElement::Element{e: next_element.expect("Element should be initialized").into()});



    for i in 0..root.child_nodes().length() {
        let child = root.child_nodes().get(i).unwrap();
        break_down_node(&child, seq);
    }

    seq.push(TypeElement::EndElement());

    seq
}

pub fn intro_text() -> HtmlElement<P> {
    view! {
        // TODO make orange name link to github
        <p>"Hi, I'm "<a href="https://github.com/mxgordon" rel="noreferrer noopener" target="_blank" class="orange">"Max Gordon"</a>" and this is my personal website. Please explore it!"</p>
    }
}

#[component]
pub fn TypeWriter(#[prop(into)] html_to_type: HtmlElement<AnyElement>, #[prop(default=div().into(), into)] base_element: HtmlElement<AnyElement>) -> impl IntoView {
    let container_div_ref = create_node_ref::<AnyElement>();
    
    let mut charSeq: Vec<TypeElement> = Vec::new();

    break_down_html(&html_to_type.clone(), &mut charSeq);

    container_div_ref.on_load( |e| {
        let _ = e.on_mount(move |e| {
            let idxRef = RefCell::new(0);
            let intervalHandleRef: Rc<RefCell<Option<IntervalHandle>>> = Rc::new(RefCell::new(None));
            
            match &charSeq[0] {
                TypeElement::Element{e} => {
                    let _ = e.clone().classes("typing");
                }
                el => {
                    log!("Unexpected element: {:?}", el);
                }
            }

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
                            match &charSeq[0] {
                                TypeElement::Element{e} => {
                                    let classes = e.clone().class_name();
                                    let _ = e.clone().attr("class", classes.replace("typing", ""));
                                }
                                el => {
                                    log!("Unexpected element: {:?}", el);
                                }
                            }

                            intervalHandleRef.borrow_mut().unwrap().clear();
                            return;
                        }

                        let next = &charSeq[*idx];

                        match next {
                            TypeElement::EndElement() => {
                                let parent_node = current_element.parent_element().unwrap();
                                *current_element = ToHtmlElement::to_leptos_element(&parent_node);
                            }
                            TypeElement::Character{c} => {
                                let mut text = current_text.as_ref().unwrap().text_content().unwrap_or_default();
    
                                text.push(*c);
    
                                current_text.as_ref().unwrap().set_text_content(Some(&text));

                                iter_again = false;
                            }
                            TypeElement::Element{e} => {
                                let _ = current_element.append_child(&e);
                                *current_element = e.clone();
                            }
                            TypeElement::StartText() => {
                                let result = current_element.append_with_str_1("");
    
                                log!("appended text: {:?}", result);
    
                                *current_text = Some(current_element.last_child().unwrap());  // unwrapping then Some, so it'll panic if something weird happens
                            }
                            TypeElement::EndText() => {
                                *current_text = None;
                            }
                        }
    
                        *idx += 1;
                    }
                }
            };
            *intervalHandleRef.borrow_mut() = set_interval_with_handle(cb, Duration::from_millis(10)).ok();
        });
    });

    base_element.node_ref(container_div_ref)
}
