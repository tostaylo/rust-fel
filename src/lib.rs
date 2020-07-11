mod a_child_component;
mod a_component;
mod app;
mod js;
mod list;
mod list_item;
mod list_item_1;
mod list_item_2;
mod list_item_3;
mod main_child;
mod main_component;
mod reducer;
mod state;
mod use_component;
mod use_component_child;
use js::log;
mod rustact;
use crate::main_component::Main;
use crate::state::State;
use app::app;
use rustact::Component;
use std::cell::RefCell;
use std::rc::Rc;
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
    let main = Main::create(5);
    let app = rustact::App::new(main);
    app.mount();

    Ok(())
}
