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

        let header = Element::new("h1".to_owned(), {
            let mut props = Props {
                text: Some("rust-fel Playwright demo".to_owned()),
                ..Default::default()
            };
            props.set_attribute("data-cy", "demo-title");
            props
        });

        let status = Element::new("span".to_owned(), {
            let mut props = Props {
                text: Some(count.to_string()),
                ..Default::default()
            };
            props.set_attribute("data-cy", "count-value");
            props
        });

        let increment_button = Element::new("button".to_owned(), {
            let mut props = Props {
                text: Some("Increment".to_owned()),
                on_click: Some(increment),
                ..Default::default()
            };
            props.set_attribute("type", "button");
            props.set_attribute("data-cy", "increment");
            props
        });

        let decrement_button = Element::new("button".to_owned(), {
            let mut props = Props {
                text: Some("Decrement".to_owned()),
                on_click: Some(decrement),
                ..Default::default()
            };
            props.set_attribute("type", "button");
            props.set_attribute("data-cy", "decrement");
            props
        });

        let controls = Element::new("div".to_owned(), {
            let mut props = Props {
                children: Some(vec![increment_button, decrement_button]),
                ..Default::default()
            };
            props.set_attribute("class", "controls");
            props
        });

        Element::new("main".to_owned(), {
            let mut props = Props {
                children: Some(vec![header, status, controls]),
                ..Default::default()
            };
            props.set_attribute("id", id);
            props.set_attribute("class", "demo-app");
            props
        })
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let app = App::new(Handle::<Counter>::create());
    app.mount("root");
    Ok(())
}
