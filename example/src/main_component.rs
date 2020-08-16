use crate::action::Action;
use crate::handle;
use crate::main_child::{ChildProps, MainChild};
use crate::main_sibling::MainSibling;
use rust_fel;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

#[derive(Debug, Default, Clone)]
pub struct MainState {
    count: i32,
    input_val: String,
}

pub enum Actions {
    Counter(Action),
    InputVal,
}

#[derive(Debug, Default, Clone)]
pub struct Main {
    child: handle::Handle<MainChild>,
    child_sibling: handle::Handle<MainSibling>,
    id: String,
    state: MainState,
    props: String,
}

impl Main {
    pub fn create() -> handle::Handle<Self> {
        let main = Main {
            id: "main".to_owned(),
            state: MainState {
                count: 0,
                input_val: "".to_owned(),
            },
            child: MainChild::create(),
            child_sibling: MainSibling::create(),
            ..Default::default()
        };
        handle::Handle(Rc::new(RefCell::new(main)))
    }
}

impl rust_fel::Component for handle::Handle<Main> {
    type Properties = String;
    type Message = Actions;
    type State = MainState;

    fn add_props(&mut self, props: Self::Properties) {
        self.0.borrow_mut().props = props;
    }

    fn reduce_state(&mut self, message: Actions) {
        match message {
            Actions::Counter(Action::Increment) => self.0.borrow_mut().state.count += 100,
            Actions::Counter(Action::Decrement) => self.0.borrow_mut().state.count -= 100,
            Actions::InputVal => {
                let window = web_sys::window().expect("no global `window` exists");
                let document = window.document().expect("should have a document on window");

                let input = document
                    .get_element_by_id("input-el")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlInputElement>()
                    .expect("should be");

                self.0.borrow_mut().state.input_val = input.value().to_owned();
            }
        }

        rust_fel::re_render(self.render(), Some(self.0.borrow().id.clone()));
    }

    fn render(&self) -> rust_fel::Element {
        let mut clone = self.clone();
        let mut clone2 = self.clone();
        let mut clone3 = self.clone();
        let mut borrow = self.0.borrow_mut();
        let state = borrow.state.clone();
        let closure = Rc::new(RefCell::new(move || {
            clone2.reduce_state(Actions::Counter(Action::Decrement))
        }));

        let child_props = ChildProps {
            input_props: state.input_val.clone(),
            counter_props: state.count.to_string(),
            closure: Some(closure),
        };

        borrow.child.add_props(child_props);

        let main_text = rust_fel::html(format!(
            "<span | data-cy=main-text| >Main {}</span>",
            state.count.to_string()
        ));

        let inc_button_text = rust_fel::Element::new(
            "TEXT_ELEMENT".to_owned(),
            rust_fel::Props {
                text: Some("Increment".to_owned()),
                ..Default::default()
            },
        );

        let send_button_text = rust_fel::Element::new(
            "TEXT_ELEMENT".to_owned(),
            rust_fel::Props {
                text: Some("Send".to_owned()),
                ..Default::default()
            },
        );

        let inc_button = rust_fel::Element::new(
            "button".to_owned(),
            rust_fel::Props {
                on_click: Some(Box::new(move || {
                    clone3.reduce_state(Actions::Counter(Action::Increment))
                })),
                data_cy: Some("increment-main".to_owned()),
                children: Some(vec![inc_button_text]),
                ..Default::default()
            },
        );

        let send_button = rust_fel::Element::new(
            "button".to_owned(),
            rust_fel::Props {
                on_click: Some(Box::new(move || clone.reduce_state(Actions::InputVal))),
                children: Some(vec![send_button_text]),
                data_cy: Some("send-input-val".to_owned()),
                ..Default::default()
            },
        );

        let input = rust_fel::html("<input | id=input-el type=text |></input>".to_owned());
        let input_wrapper = rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                class_name: Some("input-wrapper".to_owned()),
                children: Some(vec![input, send_button]),
                ..Default::default()
            },
        );
        let main_el = rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                class_name: Some("main-el".to_owned()),
                children: Some(vec![main_text, inc_button, input_wrapper]),
                ..Default::default()
            },
        );
        let child_wrapper = rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                class_name: Some("child-wrapper".to_owned()),
                children: Some(vec![borrow.child.render(), borrow.child_sibling.render()]),
                ..Default::default()
            },
        );

        let main = rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                id: Some(borrow.id.clone()),
                class_name: Some("main".to_owned()),
                children: Some(vec![main_el, child_wrapper]),
                ..Default::default()
            },
        );

        main
    }
}
