use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Element<'a> {
    html_type: &'a str,
    props: Props<'a>,
}

pub struct Props<'a> {
    pub children: Option<Vec<Element<'a>>>,
    pub text: Option<&'a str>,
    pub on_click: Option<&'static dyn Fn() -> ()>,
    pub class_name: Option<&'a str>,
}

impl<'a> Default for Props<'_> {
    fn default() -> Self {
        Props {
            children: None,
            text: None,
            on_click: None,
            class_name: None,
        }
    }
}

impl<'a> Element<'_> {
    pub fn new(html_type: &'a str, props: Props<'a>) -> Element<'a> {
        Element { html_type, props }
    }
}

pub fn render(rustact_element: &Element, container: &web_sys::Node) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    if rustact_element.html_type == "TEXT_ELEMENT" {
        match rustact_element.props.text.as_ref() {
            Some(text) => {
                let dom = container
                    .append_child(&document.create_text_node(&text))
                    .expect("couldn't append text node");

                match rustact_element.props.children.as_ref() {
                    Some(children) => {
                        for child in children.iter() {
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
                dom_el.set_class_name(class_name);
            }
            None => (),
        }

        match rustact_element.props.on_click.as_ref() {
            Some(&on_click) => {
                let closure = Closure::wrap(Box::new(move || on_click()) as Box<dyn Fn()>);
                dom_el
                    .dyn_ref::<HtmlElement>()
                    .expect("#loading should be an `HtmlElement`")
                    .set_onclick(Some(closure.as_ref().unchecked_ref()));
                closure.forget();
            }
            None => (),
        }

        let dom = container
            .append_child(&dom_el)
            .expect("couldn't append child");

        match rustact_element.props.children.as_ref() {
            Some(children) => {
                for child in children.iter() {
                    render(child, &dom)
                }
            }
            None => (),
        }
    }
}

pub fn create_element<'a>(html_type: &'a str, props: Props<'a>) -> Element<'a> {
    Element::new(html_type, props)
}
