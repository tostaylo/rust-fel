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
}

impl<'a> Default for Props<'_> {
    fn default() -> Self {
        Props {
            children: None,
            text: None,
            on_click: None,
        }
    }
}

impl<'a> Element<'_> {
    pub fn new(html_type: &'a str, props: Props<'a>) -> Element<'a> {
        Element { html_type, props }
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
        let dom_el = document.create_element(&element.html_type).unwrap();
        match element.props.on_click.as_ref() {
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

pub fn create_element<'a>(html_type: &'a str, props: Props<'a>) -> Element<'a> {
    Element::new(html_type, props)
}
