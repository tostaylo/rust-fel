use crate::{create_element, set_state, Element, Props};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn app<'a>(props: bool) -> Element<'a> {
    let hi_text = create_element(
        "TEXT_ELEMENT",
        Props {
            text: Some("Hi from rustact"),
            ..Default::default()
        },
    );

    let bye_text = create_element(
        "TEXT_ELEMENT",
        Props {
            text: Some("Bye from rustact"),
            ..Default::default()
        },
    );
    fn logs_on_click() {
        log("I'm list item one");
    }

    let list_item_1 = create_element(
        "li",
        Props {
            children: Some(vec![hi_text]),
            on_click: Some(&logs_on_click),
            ..Default::default()
        },
    );

    let list_item_2 = create_element(
        "li",
        Props {
            children: Some(vec![bye_text]),
            on_click: Some(&set_state),
            ..Default::default()
        },
    );

    let list_items = match props {
        true => vec![list_item_1, list_item_2],
        false => vec![list_item_2, list_item_1],
    };

    let list = create_element(
        "ul",
        Props {
            children: Some(list_items),
            ..Default::default()
        },
    );

    let app_title = create_element(
        "TEXT_ELEMENT",
        Props {
            text: Some("rustact"),
            ..Default::default()
        },
    );

    let app = create_element(
        "div",
        Props {
            class_name: Some("app"),
            children: Some(vec![app_title, list]),
            ..Default::default()
        },
    );

    app
}
