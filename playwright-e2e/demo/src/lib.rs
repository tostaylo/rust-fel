use rust_fel::{re_render, App, ClosureProp, Component, Element};
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

        rust_fel::rsx! {
            <main id={id} class="demo-app">
                <h1 data-cy="demo-title">{"rust-fel Playwright demo"}</h1>
                <span data-cy="count-value">{count}</span>
                <div class="controls">
                    <button type="button" data-cy="increment" on_click={increment}>{"Increment"}</button>
                    <button type="button" data-cy="decrement" on_click={decrement}>{"Decrement"}</button>
                </div>
            </main>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let app = App::new(Handle::<Counter>::create());
    app.mount("root");
    Ok(())
}
