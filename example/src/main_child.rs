use crate::action::Action;
use crate::grand_child::{GCProps, GrandChild};
use crate::handle;
use rust_fel;
use std::cell::RefCell;
use std::fmt;
use std::ops::DerefMut;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct ChildProps {
    pub input_props: String,
    pub counter_props: String,
    pub closure: Option<Rc<RefCell<dyn FnMut()>>>,
}

#[derive(Debug, Default, Clone)]
pub struct MainChild {
    state: i32,
    props: ChildProps,
    id: String,
    child: handle::Handle<GrandChild>,
}

impl fmt::Debug for ChildProps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?} this is a mainchild props", self.counter_props)
    }
}

impl MainChild {
    pub fn create() -> handle::Handle<Self> {
        let main_child = MainChild {
            child: GrandChild::create(),
            id: "main-child".to_owned(),
            ..Default::default()
        };
        handle::Handle(Rc::new(RefCell::new(main_child)))
    }
}

impl rust_fel::Component for handle::Handle<MainChild> {
    type Properties = ChildProps;
    type Message = Action;
    type State = i32;

    fn add_props(&mut self, props: Self::Properties) {
        self.0.borrow_mut().props = props;
    }

    fn reduce_state(&mut self, message: Action) {
        match message {
            Action::Increment => self.0.borrow_mut().state += 1,
            Action::Decrement => self.0.borrow_mut().state -= 1,
        }

        rust_fel::re_render(self.render(), Some(self.0.borrow().id.clone()));
    }

    fn render(&self) -> rust_fel::Element {
        let mut clone = self.clone();
        let borrow = self.0.borrow();
        let mut child = borrow.child.clone();
        let state = borrow.state.clone();
        let borrow_clone = borrow.clone();
        let closure_prop = borrow_clone.props.closure.unwrap();
        let rc_closure_prop = Rc::clone(&closure_prop);
        let mut child_closure = move || clone.reduce_state(Action::Increment);

        let on_click_closure = Box::new(move || {
            let mut reference = rc_closure_prop.borrow_mut();
            let deref = reference.deref_mut();
            deref();
        });

        let main_text = rust_fel::html(format!(
            "<span | data-cy=main-child-text|>Main Child {}</span>",
            state.to_string()
        ));

        let grand_child_props = GCProps {
            input_props: borrow.props.input_props.clone(),
        };

        child.add_props(grand_child_props);

        let inc_button_text = rust_fel::Element::new(
            "TEXT_ELEMENT".to_owned(),
            rust_fel::Props {
                text: Some("Increment".to_owned()),
                ..Default::default()
            },
        );

        let inc_button = rust_fel::Element::new(
            "button".to_owned(),
            rust_fel::Props {
                data_cy: Some("increment-main-child".to_owned()),
                on_click: Some(Box::new(move || child_closure())),
                children: Some(vec![inc_button_text]),
                ..Default::default()
            },
        );

        let send_button_text = rust_fel::Element::new(
            "TEXT_ELEMENT".to_owned(),
            rust_fel::Props {
                text: Some("Update Parent".to_owned()),
                ..Default::default()
            },
        );
        let send_button = rust_fel::Element::new(
            "button".to_owned(),
            rust_fel::Props {
                on_click: Some(on_click_closure),
                children: Some(vec![send_button_text]),
                data_cy: Some("update-parent".to_owned()),
                ..Default::default()
            },
        );
        let button_wrapper = rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                children: Some(vec![inc_button, send_button]),
                ..Default::default()
            },
        );
        let main_el = rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                class_name: Some("main-el".to_owned()),
                children: Some(vec![main_text, button_wrapper]),
                ..Default::default()
            },
        );
        let main = rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                id: Some(self.0.borrow().id.clone()),
                class_name: Some("main-child".to_owned()),
                children: Some(vec![main_el, child.render()]),
                ..Default::default()
            },
        );

        main
    }
}
