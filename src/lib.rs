// use gloo::{events::EventListener, timers::callback::Timeout};
use wasm_bindgen::prelude::*;

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

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // let a_button = HelloButton::new(&document).expect("should be a button");
    // let button = a_button.button;

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}

struct Rustact {
    element: Element,
}
struct Element {
    html_type: String,
    props: Props,
}

struct Props {
    children: Vec<Element>,
    text: String,
}

impl Element {
    fn new(&self, html_type: String, props: Props) -> Element {
        Element { html_type, props }
    }
}

impl Rustact {
    fn render(element: Element, container: String) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        let val = document.create_element(element.html_type.as_str())?;
        val.set_inner_html("Hello from Rust!");

        body.append_child(&val)?;
        Ok(())
    }
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
