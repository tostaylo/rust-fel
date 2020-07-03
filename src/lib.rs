mod a_child_component;
mod a_component;
mod app;
mod js;
mod list;
mod list_item;
mod list_item_1;
mod list_item_2;
mod list_item_3;
mod reducer;
mod state;
mod use_component;
mod use_component_child;
use js::log;
mod rustact;
use crate::state::State;
use app::app;
use rustact::Component;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
#[macro_use]
extern crate lazy_static;

lazy_static! {
#[derive(Debug, Default, Clone, Copy)]
static ref STORE: Mutex<rustact::RustactStore<State>> = Mutex::new(rustact::RustactStore::new(State::new(true)));
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Main {
    props: i32,
}

impl rustact::Component for Main {
    type Properties = i32;
    type Message = String;

    fn create(props: Self::Properties) -> Self {
        Main { props }
    }

    fn render(&self) -> rustact::Element {
        let html = rustact::html(
            "<h5><span><span><p></p></span></span><h1><h2></h2><h3><h4></h4></h3></h1></h5>"
                .to_owned(),
        );

        let main = rustact::create_element(
            "div".to_owned(),
            rustact::Props {
                class_name: Some("app".to_owned()),
                children: Some(vec![html]),
                ..Default::default()
            },
        );

        main
    }
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // let window = web_sys::window().expect("no global `window` exists");
    // let document = window.document().expect("should have a document on window");

    // let root_node = document
    //     .get_element_by_id("root")
    //     .expect("should have a root div")
    //     .append_child(&document.create_element("div").unwrap())
    //     .expect("couldn't append child");

    // let app = app();
    // rustact::render(app, &root_node);
    let main = Main::create(5);
    let app = rustact::App::new(main);
    app.mount();

    Ok(())
}
