mod js;
mod main_child;
mod main_component;
use js::log;
mod rustact;
use crate::main_component::Main;
use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let main = Main::create();
    let app = rustact::App::new(main);
    app.mount();

    Ok(())
}
