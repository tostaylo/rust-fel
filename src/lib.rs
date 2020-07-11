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

#[derive(Debug, Default, Clone)]
pub struct Main {
    props: i32,
    child: rustact::Handle<MainChild>,
    id: String,
    count: i32,
}

impl Main {
    fn create(props: i32) -> rustact::Handle<Self> {
        log(&format!("create Main"));
        let main = Main {
            props,
            id: "main".to_owned(),
            child: MainChild::create(5),
            ..Default::default()
        };
        rustact::Handle(Rc::new(RefCell::new(main)))
    }
}

impl rustact::Component for rustact::Handle<Main> {
    type Properties = i32;
    type Message = String;

    fn set_state(&mut self, new_count: i32) {
        self.0.borrow_mut().count += new_count;
        rustact::re_render(self.render(), Some(self.0.borrow().id.clone()));
    }

    fn render(&self) -> rustact::Element {
        log(&format!("Hi, From Main {:?}", self.0.borrow().child));
        let main_text = rustact::create_element(
            "TEXT_ELEMENT".to_owned(),
            rustact::Props {
                text: Some(format!(
                    "Hi, From Main {}",
                    self.0.borrow().count.to_string()
                )),
                ..Default::default()
            },
        );
        let html = rustact::html(
            "<h5><span><span><p></p></span></span><h1><h2></h2><h3><h4></h4></h3></h1></h5>"
                .to_owned(),
        );
        let mut clone = self.clone();

        let main = rustact::create_element(
            "div".to_owned(),
            rustact::Props {
                id: Some(self.0.borrow().id.clone()),
                mouse: Some(Box::new(move || clone.set_state(2))),
                class_name: Some("main".to_owned()),
                children: Some(vec![main_text, html, self.0.borrow().child.render()]),
                ..Default::default()
            },
        );

        main
    }
}

#[derive(Debug, Default, Clone)]
pub struct MainChild {
    props: i32,
    count: i32,
    id: String,
}

impl MainChild {
    fn create(props: i32) -> rustact::Handle<Self> {
        log(&format!("create MainChild"));
        let main_child = MainChild {
            props,
            id: "main-child".to_owned(),
            ..Default::default()
        };
        rustact::Handle(Rc::new(RefCell::new(main_child)))
    }
}

impl rustact::Component for rustact::Handle<MainChild> {
    type Properties = i32;
    type Message = String;

    fn set_state(&mut self, new_count: i32) {
        self.0.borrow_mut().count += new_count;
        rustact::re_render(self.render(), Some(self.0.borrow().id.clone()));
    }

    fn render(&self) -> rustact::Element {
        log(&format!("Hi, From Main Child {:?}", self));
        let main_text = rustact::create_element(
            "TEXT_ELEMENT".to_owned(),
            rustact::Props {
                text: Some(format!(
                    "Hi, From Main Child {}",
                    self.0.borrow().count.to_string()
                )),
                ..Default::default()
            },
        );
        let html = rustact::html(
            "<h5><span><span><p></p></span></span><h1><h2></h2><h3><h4></h4></h3></h1></h5>"
                .to_owned(),
        );
        let mut clone = self.clone();
        let main = rustact::create_element(
            "div".to_owned(),
            rustact::Props {
                id: Some(self.0.borrow().id.clone()),
                on_click: Some(Box::new(move || clone.set_state(2))),
                class_name: Some("main-child".to_owned()),
                children: Some(vec![main_text, html]),
                ..Default::default()
            },
        );

        main
    }
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let main = Main::create(5);
    let app = rustact::App::new(main);
    app.mount();

    Ok(())
}
