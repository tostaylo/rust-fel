mod app;
mod js;
mod list;
mod list_item_1;
mod list_item_2;
mod reducer;
mod state;
use js::log;
mod rustact;
use crate::state::State;
use app::app;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
#[macro_use]
extern crate lazy_static;

lazy_static! {
#[derive(Debug, Default, Clone, Copy)]
static ref STORE: Mutex<rustact::RustactStore<State>> = Mutex::new(rustact::RustactStore::new(State::new(true)));
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let html = "<div><span></span><h1><h2></h2></h1></div>";
    // rustact::parse_html(html.to_owned());//
    rustact::parse_with_stack(html.to_owned());
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let root_node = document
        .get_element_by_id("root")
        .expect("should have a root div")
        .append_child(&document.create_element("div").unwrap())
        .expect("couldn't append child");

    let app = app();
    rustact::render(app, &root_node);

    Ok(())
}
