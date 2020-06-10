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
    let mut rustact_struct = rustact::Rustact::new(initial_state);
    log(&format!("{:?}", rustact_struct));

    // fn reducer(state: &State, action: &str) -> State {
    //     log(&format!("{:?} {:?} inside reduce", state, action));
    //     match action {
    //         "reverse" => State { order: false },
    //         "initial" => State { order: true },
    //         _ => State { ..state.clone() },
    //     }
    // }

    // rustact_struct.reduce(Box::new(reducer), "reverse");
    log(&format!("{:?} after reduce", rustact_struct.store));
    let app = app(rustact_struct);
    rustact::render(app, &root_node);

    Ok(())
}
