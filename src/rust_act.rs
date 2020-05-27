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
    pub children: String,
}

impl Element {
    pub fn new(html_type: String, props: Props) -> Element {
        Element { html_type, props }
    }
}

pub fn render(element: Element, container: web_sys::Element) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    if element.html_type == "TEXT_ELEMENT" {
        log("hi from");
        container
            .append_child(&document.create_text_node(&element.props.children))
            .expect("couldn't append text node");
    } else {
        container
            .append_child(
                &document
                    .create_element(&element.html_type)
                    .expect("couldn't append html"),
            )
            .expect("couldn't append child");
    }
}

pub fn create_element(html_type: String, props: Props) -> Element {
    Element::new(html_type, props)
}
