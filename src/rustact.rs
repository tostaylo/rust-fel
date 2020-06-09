use crate::app;
use crate::log;
use crate::reducer::State;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub struct Element {
    html_type: String,
    props: Props,
}

pub struct Props {
    pub children: Option<Vec<Element>>,
    pub text: Option<String>,
    pub on_click: Option<Box<dyn FnMut() -> ()>>,
    pub class_name: Option<String>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            children: None,
            text: None,
            on_click: None,
            class_name: None,
        }
    }
}

impl Element {
    pub fn new(html_type: String, props: Props) -> Element {
        Element { html_type, props }
    }
}

pub fn render(rustact_element: Element, container: &web_sys::Node) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    if rustact_element.html_type == "TEXT_ELEMENT" {
        match rustact_element.props.text {
            Some(text) => {
                let dom = container
                    .append_child(&document.create_text_node(&text))
                    .expect("couldn't append text node");

                match rustact_element.props.children {
                    Some(children) => {
                        for child in children {
                            render(child, &dom)
                        }
                    }
                    None => (),
                }
            }
            None => (),
        };
    } else {
        let dom_el = document.create_element(&rustact_element.html_type).unwrap();

        match rustact_element.props.class_name {
            Some(class_name) => {
                dom_el.set_class_name(&class_name);
            }
            None => (),
        }

        match rustact_element.props.on_click {
            Some(mut on_click) => {
                let closure = Closure::wrap(Box::new(move || on_click()) as Box<dyn FnMut()>);
                dom_el
                    .dyn_ref::<HtmlElement>()
                    .expect("should be an `HtmlElement`")
                    .set_onclick(Some(closure.as_ref().unchecked_ref()));
                closure.forget();
            }
            None => (),
        }

        let dom = container
            .append_child(&dom_el)
            .expect("couldn't append child");

        match rustact_element.props.children {
            Some(children) => {
                for child in children {
                    render(child, &dom)
                }
            }
            None => (),
        }
    }
}

pub fn create_element(html_type: String, props: Props) -> Element {
    Element::new(html_type, props)
}

// pub fn use_reducer(initial_state: &'static State) -> (&State, Box<dyn FnMut(&str) -> ()>) {
//     let message_1 = format!("here is state initially {:?}", initial_state);
//     js::log(&message_1);
//     let mut state = initial_state;

//     let dispatch = Box::new(move |action: &str| {
//         state = &state.reduce(action);
//         let message_dispatch = format!("here is state in dispatch {:?}", state);

//         js::log(&message_dispatch);
//         if initial_state.order == false {
//             re_render();
//         }
//         ()
//     });
//     let message_2 = format!("here is state after dispatch {:?}", state);
//     js::log(&message_2);
//     (state, dispatch)
// }

pub fn rustact() -> Box<dyn FnMut(i32) -> (Rc<RefCell<i32>>, Box<dyn FnMut(i32) -> ()>)> {
    let internal_state = Rc::new(RefCell::new(0));
    let internal_state_copy = internal_state.clone();

    let use_state = move |initial_state: i32| {
        let val: i32;

        if *internal_state_copy.borrow() > 0 {
            val = *internal_state_copy.borrow();
            log(&format!("{:?} setting val", internal_state));
        } else {
            *internal_state.borrow_mut() = initial_state;
            val = initial_state;
            log(&format!("{:?} setting internal", internal_state));
        }

        let state = Rc::new(RefCell::new(val));
        let state_copy = state.clone();
        let set_state = Box::new(move |new_val: i32| {
            *state_copy.borrow_mut() += new_val;
        }) as Box<dyn FnMut(i32) -> ()>;

        (state, set_state)
    };
    return Box::new(use_state)
        as Box<dyn FnMut(i32) -> (Rc<RefCell<i32>>, Box<dyn FnMut(i32) -> ()>)>;
}

pub fn re_render(state: State) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let root = document
        .get_element_by_id("root")
        .expect("should have a root div");
    let node = root.first_child().unwrap();

    root.remove_child(&node).expect("unable to remove child");

    let root_node = root
        .append_child(&document.create_element("div").unwrap())
        .expect("couldn't append child");
    render(app(state), &root_node);
}
