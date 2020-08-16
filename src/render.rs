use crate::element::Element;
use crate::props::ClosureProp;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

/// Recursively builds a DOM tree.
///
/// # Arguments
///
/// * `rust_fel_element` - A [rust_fel::Element](../element/struct.Element.html)
/// * `container` - A reference to a [web_sys::Node](https://docs.rs/web-sys/0.3.21/web_sys/struct.Node.html)
/// * `is_update` - A boolean allowing the function to differentiate between first mount of the application and subsequent updates.
///
/// # Examples
/// Taken from [rust_fel::App::{mount}](../app/struct.App.html#method.mount)  
/// ```ignore
///  let el = self.component.render();
///  let window = web_sys::window().expect("no global `window` exists");
///  let document = window.document().expect("should have a document on window");
///
///  let root_node = document
///      .get_element_by_id("root")
///      .expect("should have a root div");
///
///  render(el, &root_node, false);
/// ```

pub fn render(rust_fel_element: Element, container: &web_sys::Node, is_update: bool) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    if rust_fel_element.html_type == "TEXT_ELEMENT" {
        match rust_fel_element.props.text {
            Some(text) => {
                container
                    .append_child(&document.create_text_node(&text))
                    .expect("couldn't append text node");
            }
            None => (),
        };
    } else {
        let dom_el = document
            .create_element(&rust_fel_element.html_type)
            .unwrap();

        match rust_fel_element.props.class_name {
            Some(class_name) => {
                dom_el.set_class_name(&class_name);
            }
            None => (),
        }

        match rust_fel_element.props.href {
            Some(href) => {
                dom_el
                    .set_attribute("href", &href)
                    .expect("could not set href");
            }
            None => (),
        }

        match rust_fel_element.props.src {
            Some(src) => {
                dom_el
                    .set_attribute("src", &src)
                    .expect("could not set src");
            }
            None => (),
        }

        match rust_fel_element.props.type_attr {
            Some(type_attr) => {
                dom_el
                    .set_attribute("type", &type_attr)
                    .expect("could not set type");
            }
            None => (),
        }

        match rust_fel_element.props.data_cy {
            Some(data_cy) => {
                dom_el
                    .set_attribute("data-cy", &data_cy)
                    .expect("could not set data-cy");
            }
            None => (),
        }

        match rust_fel_element.props.role {
            Some(role) => {
                dom_el
                    .set_attribute("role", &role)
                    .expect("could not set role");
            }
            None => (),
        }

        match rust_fel_element.props.on_click {
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

        match rust_fel_element.props.mouse {
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
        match rust_fel_element.props.id {
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

        match rust_fel_element.props.children {
            Some(children) => {
                for child in children {
                    render(child, &dom, false)
                }
            }
            None => (),
        }
    }
}

/// After first mount this funtion will update the virtual DOM and then the real DOM.
/// It differs from render in that render is only used in mount of the app
/// # Arguments
///
/// * `rust_fel_element` - A [rust_fel::Element](../element/struct.Element.html)
/// * `id` - A [String](https://doc.rust-lang.org/std/string/struct.String.html) wrapped in an [Option](https://doc.rust-lang.org/std/option/enum.Option.html)
///
/// # Examples
/// ```ignore
/// // Used when a rust_fel struct component updates it's state and wants to propagate the changes
/// // to it's children
///    fn reduce_state(&mut self, message: Action) {
///       match message {
///             Action::Increment => self.0.borrow_mut().state += 5,
///             Action::Decrement => self.0.borrow_mut().state -= 5,
///         }
///
///         rust_fel::re_render(self.render(), Some(self.0.borrow().id.clone()));
///     }
/// ```

pub fn re_render(rust_fel_element: Element, id: Option<String>) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    if let Some(i) = id {
        let child = document
            .get_element_by_id(&i)
            .expect("should have a root div");

        let parent = child.parent_node().unwrap();

        render(rust_fel_element, &parent, true);
    } else {
        panic!("Components that initalize re-renders must have a Id");
    }
}
