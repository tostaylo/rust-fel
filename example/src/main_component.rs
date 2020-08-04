use crate::action::Action;
use crate::handle;
use crate::main_child::{ChildProps, MainChild};
use crate::main_sibling::{ChildProps as MainSiblingChildProps, MainSibling};
use crate::text_wrapper::text_wrapper;
use rust_fel;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct Main {
    child: handle::Handle<MainChild>,
    child_sibling: handle::Handle<MainSibling>,
    id: String,
    state: i32,
    props: String,
}

impl Main {
    pub fn create() -> handle::Handle<Self> {
        let main = Main {
            id: "main".to_owned(),
            state: 0,
            child: MainChild::create(),
            child_sibling: MainSibling::create(),
            ..Default::default()
        };
        handle::Handle(Rc::new(RefCell::new(main)))
    }
}

impl rust_fel::Component for handle::Handle<Main> {
    type Properties = String;
    type Message = Action;
    type State = i32;

    fn add_props(&mut self, props: Self::Properties) {
        self.0.borrow_mut().props = props;
    }

    fn reduce_state(&mut self, message: Action) {
        match message {
            Action::Increment => self.0.borrow_mut().state += 100,
            Action::Decrement => self.0.borrow_mut().state -= 100,
        }

        rust_fel::re_render(self.render(), Some(self.0.borrow().id.clone()));
    }

    fn render(&self) -> rust_fel::Element {
        let mut clone = self.clone();
        let mut clone2 = self.clone();
        let mut borrow = self.0.borrow_mut();
        let state = borrow.state.clone();
        let closure = Rc::new(RefCell::new(move || clone2.reduce_state(Action::Decrement)));

        let child_props = ChildProps {
            string_props: state.to_string(),
            closure: Some(closure),
        };

        let child_sibling_props = MainSiblingChildProps {
            string_props: state.to_string(),
        };

        borrow.child.add_props(child_props);
        borrow.child_sibling.add_props(child_sibling_props);

        let main_text = rust_fel::create_element(
            "TEXT_ELEMENT".to_owned(),
            rust_fel::Props {
                text: Some(format!("Hi, From Main {}", state.to_string())),
                ..Default::default()
            },
        );
        let main_el = text_wrapper(
            "div".to_owned(),
            Some(vec![main_text]),
            None,
            Some("main-text".to_owned()),
        );

        let more_text = rust_fel::create_element(
            "TEXT_ELEMENT".to_owned(),
            rust_fel::Props {
                text: Some(format!("Hi, From More {}", state.to_string())),
                ..Default::default()
            },
        );

        let more_el = text_wrapper(
            "div".to_owned(),
            Some(vec![more_text]),
            None,
            Some("main-text".to_owned()),
        );

        let main = rust_fel::create_element(
            "div".to_owned(),
            rust_fel::Props {
                id: Some(borrow.id.clone()),
                mouse: Some(Box::new(move || clone.reduce_state(Action::Increment))),
                class_name: Some("main".to_owned()),
                children: Some(vec![
                    main_el,
                    more_el,
                    borrow.child.render(),
                    borrow.child_sibling.render(),
                ]),
                ..Default::default()
            },
        );

        main
    }
}
