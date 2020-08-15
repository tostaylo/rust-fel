use crate::action::Action;
use crate::handle;
use rust_fel;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct ChildProps {
    pub string_props: String,
}

#[derive(Debug, Default, Clone)]
pub struct MainSibling {
    state: i32,
    props: ChildProps,
    id: String,
}

impl MainSibling {
    pub fn create() -> handle::Handle<Self> {
        let main_child = MainSibling {
            id: "main-sibling".to_owned(),
            ..Default::default()
        };
        handle::Handle(Rc::new(RefCell::new(main_child)))
    }
}

impl rust_fel::Component for handle::Handle<MainSibling> {
    type Properties = ChildProps;
    type Message = Action;
    type State = i32;

    fn add_props(&mut self, props: Self::Properties) {
        self.0.borrow_mut().props = props;
    }

    fn reduce_state(&mut self, message: Action) {
        match message {
            Action::Increment => self.0.borrow_mut().state += 5,
            Action::Decrement => self.0.borrow_mut().state -= 5,
        }

        rust_fel::re_render(self.render(), Some(self.0.borrow().id.clone()));
    }

    fn render(&self) -> rust_fel::Element {
        let mut clone = self.clone();
        let borrow = self.0.borrow();

        let main_text = rust_fel::Element::new(
            "TEXT_ELEMENT".to_owned(),
            rust_fel::Props {
                text: Some(format!("Main Child Sibling {}", borrow.state.to_string())),
                ..Default::default()
            },
        );

        let main_el = rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                children: Some(vec![main_text]),
                ..Default::default()
            },
        );

        let closure = move || clone.reduce_state(Action::Decrement);

        let main = rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                id: Some(self.0.borrow().id.clone()),
                on_click: Some(Box::new(closure)),
                class_name: Some("main-child".to_owned()),
                children: Some(vec![main_el]),
                ..Default::default()
            },
        );

        main
    }
}
