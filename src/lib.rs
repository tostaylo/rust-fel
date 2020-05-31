mod app;
mod js;
use js::log;
mod rustact;
use app::app;

use rustact::{create_element, render, set_state, Element, Props};

use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let root_element = document
        .get_element_by_id("root")
        .expect("should have a root div")
        .append_child(&document.create_element("div").unwrap())
        .expect("couldn't append child");

    render(&app(true), &root_element);

    Ok(())
}
