use rust_fel::{re_render, App, ClosureProp, Component, Element, Props};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[derive(Debug, Default, Clone)]
struct CounterState {
    count: i32,
}

#[derive(Debug, Default, Clone)]
struct Counter {
    id: String,
    state: CounterState,
}

#[derive(Debug, Clone)]
struct Handle<T>(Rc<RefCell<T>>);

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Handle<Counter> {
    fn create() -> Self {
        Self(Rc::new(RefCell::new(Counter {
            id: "counter-app".to_owned(),
            state: CounterState { count: 0 },
        })))
    }
}

impl Component for Handle<Counter> {
    type Properties = ();
    type Message = Message;
    type State = CounterState;

    fn add_props(&mut self, _props: Self::Properties) {}

    fn reduce_state(&mut self, message: Self::Message) {
        {
            let mut borrow = self.0.borrow_mut();
            match message {
                Message::Increment => borrow.state.count += 1,
                Message::Decrement => borrow.state.count -= 1,
            }
        }

        let id = self.0.borrow().id.clone();
        re_render(self.render(), Some(id));
    }

    fn render(&self) -> Element {
        let (id, count) = {
            let borrow = self.0.borrow();
            (borrow.id.clone(), borrow.state.count)
        };

        let increment: ClosureProp = {
            let mut handle = self.clone();
            Box::new(move || handle.reduce_state(Message::Increment))
        };

        let decrement: ClosureProp = {
            let mut handle = self.clone();
            Box::new(move || handle.reduce_state(Message::Decrement))
        };

        let header = Element::new(
            "h1".to_owned(),
            Props {
                text: Some("rust-fel Playwright demo".to_owned()),
                data_cy: Some("demo-title".to_owned()),
                ..Default::default()
            },
        );

        let status = Element::new(
            "span".to_owned(),
            Props {
                text: Some(count.to_string()),
                data_cy: Some("count-value".to_owned()),
                ..Default::default()
            },
        );

        let increment_button = Element::new(
            "button".to_owned(),
            Props {
                text: Some("Increment".to_owned()),
                on_click: Some(increment),
                type_attr: Some("button".to_owned()),
                data_cy: Some("increment".to_owned()),
                ..Default::default()
            },
        );

        let decrement_button = Element::new(
            "button".to_owned(),
            Props {
                text: Some("Decrement".to_owned()),
                on_click: Some(decrement),
                type_attr: Some("button".to_owned()),
                data_cy: Some("decrement".to_owned()),
                ..Default::default()
            },
        );

        let controls = Element::new(
            "div".to_owned(),
            Props {
                class_name: Some("controls".to_owned()),
                children: Some(vec![increment_button, decrement_button]),
                ..Default::default()
            },
        );

        Element::new(
            "main".to_owned(),
            Props {
                id: Some(id),
                class_name: Some("demo-app".to_owned()),
                children: Some(vec![header, status, controls]),
                ..Default::default()
            },
        )
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let app = App::new(Handle::<Counter>::create());
    app.mount("root");
    Ok(())
}
