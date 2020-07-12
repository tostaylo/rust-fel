use crate::rustact;
use crate::text_wrapper::text_wrapper;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct ChildProps {
    pub vec_props: Vec<String>,
    pub string_props: String,
}

#[derive(Debug, Default, Clone)]
pub struct MainSibling {
    state: i32,
    props: ChildProps,
    id: String,
}

impl MainSibling {
    pub fn create() -> rustact::Handle<Self> {
        let main_child = MainSibling {
            id: "main-sibling".to_owned(),
            ..Default::default()
        };
        rustact::Handle(Rc::new(RefCell::new(main_child)))
    }
}

impl rustact::Component for rustact::Handle<MainSibling> {
    type Properties = ChildProps;
    type Message = String;
    type State = i32;

    fn add_props(&mut self, props: Self::Properties) {
        self.0.borrow_mut().props = props;
    }

    fn set_state(&mut self, new_count: Self::State) {
        self.0.borrow_mut().state += new_count;
        rustact::re_render(self.render(), Some(self.0.borrow().id.clone()));
    }

    fn render(&self) -> rustact::Element {
        let mut clone = self.clone();
        let borrow = self.0.borrow();

        let main_text = rustact::create_element(
            "TEXT_ELEMENT".to_owned(),
            rustact::Props {
                text: Some(format!(
                    "Hi, From Main Child Sibling {}",
                    borrow.state.to_string()
                )),
                ..Default::default()
            },
        );

        let main_el = text_wrapper(
            "div".to_owned(),
            Some(vec![main_text]),
            None,
            Some("main-text".to_owned()),
        );

        let more_text = rustact::create_element(
            "TEXT_ELEMENT".to_owned(),
            rustact::Props {
                text: Some(format!(
                    "Hi, From Main Child Sibling More {}",
                    borrow.state.to_string()
                )),
                ..Default::default()
            },
        );

        let more_el = text_wrapper(
            "div".to_owned(),
            Some(vec![more_text]),
            None,
            Some("main-text".to_owned()),
        );

        let closure = move || clone.set_state(2);

        let vec_text_elements = borrow
            .props
            .vec_props
            .iter()
            .map(|item| {
                rustact::create_element(
                    "TEXT_ELEMENT".to_owned(),
                    rustact::Props {
                        text: Some(format!(" {:?}", item)),
                        ..Default::default()
                    },
                )
            })
            .collect::<Vec<rustact::Element>>();

        let extra_text = rustact::create_element(
            "TEXT_ELEMENT".to_owned(),
            rustact::Props {
                text: Some(format!(
                    "Hi, From Main Child Sibling Extra {:?}",
                    borrow.props
                )),
                ..Default::default()
            },
        );

        let extra_el = text_wrapper(
            "div".to_owned(),
            Some(vec![extra_text]),
            None,
            Some("main-text".to_owned()),
        );

        let vec_element = rustact::create_element(
            "div".to_owned(),
            rustact::Props {
                on_click: Some(Box::new(closure.clone())),
                class_name: Some("main-text".to_owned()),
                children: Some(vec_text_elements),
                ..Default::default()
            },
        );

        let main = rustact::create_element(
            "div".to_owned(),
            rustact::Props {
                id: Some(self.0.borrow().id.clone()),
                on_click: Some(Box::new(closure.clone())),
                class_name: Some("main-child".to_owned()),
                children: Some(vec![main_el, more_el, vec_element, extra_el]),
                ..Default::default()
            },
        );

        main
    }
}
