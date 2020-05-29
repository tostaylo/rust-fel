use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Element {
    html_type: String,
    props: Props,
}

pub struct Props {
    pub children: Option<Vec<Element>>,
    pub text: Option<String>,
}

impl Element {
    pub fn new(html_type: String, props: Props) -> Element {
        Element { html_type, props }
    }
}

impl Props {
    pub fn new(children: Option<Vec<Element>>, text: Option<String>) -> Props {
        Props { children, text }
    }
}

pub fn render(element: &Element, container: &web_sys::Node) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    if element.html_type == "TEXT_ELEMENT" {
        match element.props.text.as_ref() {
            Some(text) => {
                let dom = container
                    .append_child(&document.create_text_node(&text))
                    .expect("couldn't append text node");

                match element.props.children.as_ref() {
                    Some(children) => {
                        for child in children.iter() {
                            log("inside text loop");
                            render(child, &dom)
                        }
                    }
                    None => (),
                }
            }
            None => (),
        };
    } else {
        let dom = container
            .append_child(&document.create_element(&element.html_type).unwrap())
            .expect("couldn't append child");
        match element.props.children.as_ref() {
            Some(children) => {
                for child in children.iter() {
                    log("inside html element loop");
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

pub fn create_props(children: Option<Vec<Element>>, text: Option<String>) -> Props {
    Props::new(children, text)
}
