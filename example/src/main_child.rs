use crate::grand_child::{ChildProps as GrandChildProps, GrandChild};
use crate::handle;
use crate::text_wrapper::text_wrapper;
use rust_fel;
use std::cell::RefCell;
use std::fmt;
use std::ops::DerefMut;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct ChildProps {
    pub vec_props: Vec<String>,
    pub string_props: String,
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
        write!(f, "{:#?} this is a mainchild props", self.string_props)
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
    type Message = String;
    type State = i32;

    fn add_props(&mut self, props: Self::Properties) {
        self.0.borrow_mut().props = props;
    }

    fn set_state(&mut self, new_count: Self::State) {
        self.0.borrow_mut().state += new_count;
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
        let mut child_closure = move || clone.set_state(2);

        let on_click_closure = Box::new(move || {
            let mut reference = rc_closure_prop.borrow_mut();
            let deref = reference.deref_mut();
            deref();
            child_closure();
        });

        let main_text = rust_fel::create_element(
            "TEXT_ELEMENT".to_owned(),
            rust_fel::Props {
                text: Some(format!("Hi, From Main Child {}", state.to_string())),
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
                text: Some(format!("Hi, From Main Child More {}", state.to_string())),
                ..Default::default()
            },
        );

        let more_el = text_wrapper(
            "div".to_owned(),
            Some(vec![more_text]),
            None,
            Some("main-text".to_owned()),
        );

        let vec_text_elements = borrow
            .props
            .vec_props
            .iter()
            .map(|item| {
                rust_fel::create_element(
                    "TEXT_ELEMENT".to_owned(),
                    rust_fel::Props {
                        text: Some(format!(" {:?}", item)),
                        ..Default::default()
                    },
                )
            })
            .collect::<Vec<rust_fel::Element>>();

        let extra_text = rust_fel::create_element(
            "TEXT_ELEMENT".to_owned(),
            rust_fel::Props {
                text: Some(format!("Hi, From Main Child Extra {:?}", borrow.props)),
                ..Default::default()
            },
        );

        let extra_el = text_wrapper(
            "div".to_owned(),
            Some(vec![extra_text]),
            None,
            Some("main-text".to_owned()),
        );

        let vec_element = rust_fel::create_element(
            "div".to_owned(),
            rust_fel::Props {
                class_name: Some("main-text".to_owned()),
                children: Some(vec_text_elements),
                ..Default::default()
            },
        );

        let grand_child_props = GrandChildProps {
            vec_props: borrow.props.vec_props.clone(),
            string_props: borrow.props.string_props.clone(),
        };

        child.add_props(grand_child_props);

        let main = rust_fel::create_element(
            "div".to_owned(),
            rust_fel::Props {
                id: Some(self.0.borrow().id.clone()),
                on_click: Some(on_click_closure),
                class_name: Some("main-child".to_owned()),
                children: Some(vec![
                    main_el,
                    more_el,
                    vec_element,
                    extra_el,
                    child.render(),
                ]),
                ..Default::default()
            },
        );

        main
    }
}
