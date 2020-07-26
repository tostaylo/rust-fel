mod grand_child;
mod handle;
mod js;
mod main_child;
mod main_component;
mod main_sibling;
mod text_wrapper;
use crate::main_component::Main;
use wasm_bindgen::prelude::*;
extern crate rust_fel;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let main = Main::create();
    let app = rust_fel::App::new(main);
    app.mount();

    Ok(())
}
