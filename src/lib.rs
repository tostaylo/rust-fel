mod rustact;
use rustact::{create_element, render, Props};
use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

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

    fn log_on_click() {
        log("I'm list item two");
    }

    let list_item_2 = create_element(
        "li",
        Props {
            children: Some(vec![bye_text]),
            on_click: Some(&log_on_click),
            ..Default::default()
        },
    );

    let list = create_element(
        "ul",
        Props {
            children: Some(vec![list_item_1, list_item_2]),
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

    let root_element = document
        .get_element_by_id("root")
        .expect("should have a root div")
        .append_child(&document.create_element("div").unwrap())
        .expect("couldn't append child");

    render(&app, &root_element);

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

// use gloo::{events::EventListener, timers::callback::Timeout};

// struct HelloButton {
//     button: web_sys::Element,
//     on_click: EventListener,
// }

// impl HelloButton {
//     pub fn new(document: &web_sys::Document) -> Result<HelloButton, JsValue> {
//         // Create a `<button>` element.
//         let button = document.create_element("button")?;

//         // Listen to "click" events on the button.
//         let button2 = button.clone();
//         let on_click = EventListener::new(&button, "click", move |_event| {
//             // After a one second timeout, update the button's text content.
//             let button3 = button2.clone();
//             Timeout::new(1_000, move || {
//                 button3.set_text_content(Some("Hello from one second ago!"));
//             })
//             .forget();
//         });

//         Ok(HelloButton { button, on_click })
//     }
// }

// How to use?
// let a_button = HelloButton::new(&document).expect("should be a button");
// let button = a_button.button;
