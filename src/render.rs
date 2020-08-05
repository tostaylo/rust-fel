use crate::element::Element;
use crate::props::ClosureProp;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub fn render(rustact_element: Element, container: &web_sys::Node, is_update: bool) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    if rustact_element.html_type == "TEXT_ELEMENT" {
        match rustact_element.props.text {
            Some(text) => {
                container
                    .append_child(&document.create_text_node(&text))
                    .expect("couldn't append text node");
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

        match rustact_element.props.href {
            Some(href) => {
                dom_el
                    .set_attribute("href", &href)
                    .expect("could not set href");
            }
            None => (),
        }

        match rustact_element.props.src {
            Some(src) => {
                dom_el
                    .set_attribute("src", &src)
                    .expect("could not set src");
            }
            None => (),
        }

        match rustact_element.props.type_attr {
            Some(type_attr) => {
                dom_el
                    .set_attribute("type", &type_attr)
                    .expect("could not set type");
            }
            None => (),
        }

        match rustact_element.props.role {
            Some(role) => {
                dom_el
                    .set_attribute("role", &role)
                    .expect("could not set role");
            }
            None => (),
        }

        match rustact_element.props.on_click {
            Some(mut on_click) => {
                let closure = Closure::wrap(Box::new(move || on_click()) as ClosureProp);
                dom_el
                    .dyn_ref::<HtmlElement>()
                    .expect("should be an `HtmlElement`")
                    .set_onclick(Some(closure.as_ref().unchecked_ref()));
                closure.forget();
            }
            None => (),
        }

        match rustact_element.props.mouse {
            Some(mut mouse) => {
                let closure = Closure::wrap(Box::new(move || mouse()) as ClosureProp);
                dom_el
                    .dyn_ref::<HtmlElement>()
                    .expect("should be an `HtmlElement`")
                    .add_event_listener_with_callback("mouseout", closure.as_ref().unchecked_ref())
                    .expect("could not add event listenter");
                closure.forget();
            }
            None => (),
        }

        let mut id_copy = None;
        match rustact_element.props.id {
            Some(id) => {
                dom_el.set_id(&id);

                // Is this really necessary. Kinda ugly
                id_copy = Some(id);
            }
            None => (),
        }

        // Update or first render?
        let dom;
        if is_update == true {
            let id = &id_copy.unwrap();
            let formatted = format!("#{}", id);
            let old_child = document
                .query_selector_all(&formatted)
                .expect("can't find node")
                .item(0)
                .unwrap();

            // Here we replace instead of append
            // We do this because we need to keep an element position in the dom
            container
                .replace_child(&dom_el, &old_child)
                .expect(" can't replace");

            let new_child = document
                .query_selector_all(&formatted)
                .expect("can't find node")
                .item(0)
                .unwrap();
            dom = new_child;
        } else {
            // Here we append instead or replace
            // This only occurs on first render of the app.
            dom = container
                .append_child(&dom_el)
                .expect("couldn't append child");
        };

        match rustact_element.props.children {
            Some(children) => {
                for child in children {
                    render(child, &dom, false)
                }
            }
            None => (),
        }
    }
}

pub fn re_render(app: Element, id: Option<String>) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    if let Some(i) = id {
        let child = document
            .get_element_by_id(&i)
            .expect("should have a root div");

        let parent = child.parent_node().unwrap();

        render(app, &parent, true);
    } else {
        panic!("Components that initalize re-renders must have a Id");
    }
}
