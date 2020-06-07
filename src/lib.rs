mod app;
mod js;
mod list;
mod reducer;
use js::log;
mod rustact;
use crate::reducer::State;
use app::app;
use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let root_node = document
        .get_element_by_id("root")
        .expect("should have a root div")
        .append_child(&document.create_element("div").unwrap())
        .expect("couldn't append child");
    let initial_state = State { order: true };
    let app = app(initial_state);
    rustact::render(app, &root_node);

    Ok(())
}
