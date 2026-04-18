use crate::element::Element;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, Node};

/// Recursively builds a DOM tree from a Virtual DOM [rust_fel::Element](../element/struct.Element.html).
///
/// # Arguments
///
/// * `rust_fel_element` - A [rust_fel::Element](../element/struct.Element.html)
/// * `container` - A reference to a [web_sys::Node](https://docs.rs/web-sys/0.3.21/web_sys/struct.Node.html)
/// * `is_update` - A boolean allowing the function to differentiate between first mount of the application and subsequent updates.
#[doc(hidden)]
pub fn render(rust_fel_element: Element, container: &Node, is_update: bool) {
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

        match rust_fel_element.props.text.as_ref() {
            Some(text) => {
                dom_el
                    .append_child(&document.create_text_node(text))
                    .expect("couldn't append text node");
            }
            None => (),
        };

        match rust_fel_element.props.attributes.as_ref() {
            Some(attributes) => {
                for (name, value) in attributes {
                    dom_el
                        .set_attribute(name, value)
                        .unwrap_or_else(|_| panic!("could not set attribute {}", name));
                }
            }
            None => (),
        }

        let id_copy = rust_fel_element.props.attribute("id").map(str::to_owned);

        match rust_fel_element.props.on_click {
            Some(on_click) => {
                let closure = Closure::wrap(on_click);
                dom_el
                    .dyn_ref::<HtmlElement>()
                    .expect("should be an `HtmlElement`")
                    .set_onclick(Some(closure.as_ref().unchecked_ref()));
                closure.forget();
            }
            None => (),
        }

        match rust_fel_element.props.mouse {
            Some(mouse) => {
                let closure = Closure::wrap(mouse);
                dom_el
                    .dyn_ref::<HtmlElement>()
                    .expect("should be an `HtmlElement`")
                    .add_event_listener_with_callback("mouseout", closure.as_ref().unchecked_ref())
                    .expect("could not add event listener");
                closure.forget();
            }
            None => (),
        }

        // Update or first render?
        let dom: Node = if is_update {
            let id = id_copy.unwrap();
            let old_child = document
                .get_element_by_id(id.as_str())
                .unwrap_or_else(|| panic!("Unable to get element by id {}", id));

            // Here we replace instead of append
            // We do this because we need to keep an element position in the dom
            // Possible fastest method? https://stackoverflow.com/a/22966637
            container
                .replace_child(&dom_el, &old_child)
                .expect("Unable to replace child");

            Node::from(
                document
                    .get_element_by_id(id.as_str())
                    .unwrap_or_else(|| panic!("Unable to get element by id {}", id)),
            )
        } else {
            // Here we append_child instead of replace_child
            // Replace_child only happens to the element starting the update

            container
                .append_child(&dom_el)
                .expect("Unable to append child to the container node")
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
/// Used when a ```rust_fel``` [struct](https://doc.rust-lang.org/std/keyword.struct.html) component updates it's state and wants to propagate the changes
/// to it's children.    
/// After first mount this function will update the Virtual [DOM](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Introduction) and then the real [DOM](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Introduction).  
/// It works by
///1. Passing the function a new [rust_fel::Element](../rust_fel/struct.Element.html) who invoked ```re_render``` by updating itself.
///2. Finding the associated [DOM Element](https://developer.mozilla.org/en-US/docs/Web/API/Element) by ```id```.
///3. Removing the [DOM Node](https://developer.mozilla.org/en-US/docs/Web/API/Node) and all of it's children.
///4. Replacing the removed [DOM Node](https://developer.mozilla.org/en-US/docs/Web/API/Node) with the new [rust_fel::Element](../rust_fel/struct.Element.html).
/// # Arguments
///
/// * `rust_fel_element` - A [rust_fel::Element](../element/struct.Element.html)
/// * `id` - A [String](https://doc.rust-lang.org/std/string/struct.String.html) wrapped in an [Option](https://doc.rust-lang.org/std/option/enum.Option.html)
///
/// # Examples
/// ```no_run
/// use rust_fel::{Component, Element, Props};
///
/// struct Counter {
///     id: String,
///     count: i32,
/// }
///
/// enum Action {
///     Increment,
///     Decrement,
/// }
///
/// impl Component for Counter {
///     type Properties = ();
///     type Message = Action;
///     type State = i32;
///
///     fn render(&self) -> Element {
///         let mut props = Props {
///             text: Some(self.count.to_string()),
///             ..Default::default()
///         };
///         props.set_attribute("id", self.id.clone());
///
///         Element::new(
///             "div".to_owned(),
///             props,
///         )
///     }
///
///     fn reduce_state(&mut self, message: Self::Message) {
///         match message {
///             Action::Increment => self.count += 1,
///             Action::Decrement => self.count -= 1,
///         }
///
///         rust_fel::re_render(self.render(), Some(self.id.clone()));
///     }
///
///     fn add_props(&mut self, _props: Self::Properties) {}
/// }
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
