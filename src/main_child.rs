use crate::rustact;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct ChildProps {
    pub vec_props: Vec<String>,
    pub string_props: String,
}

#[derive(Debug, Default, Clone)]
pub struct MainChild {
    state: i32,
    props: ChildProps,
    id: String,
}

impl MainChild {
    pub fn create() -> rustact::Handle<Self> {
        let main_child = MainChild {
            id: "main-child".to_owned(),
            ..Default::default()
        };
        rustact::Handle(Rc::new(RefCell::new(main_child)))
    }
}

impl rustact::Component for rustact::Handle<MainChild> {
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
                text: Some(format!("Hi, From Main Child {}", borrow.state.to_string())),
                ..Default::default()
            },
        );
        let more_text = rustact::create_element(
            "TEXT_ELEMENT".to_owned(),
            rustact::Props {
                text: Some(format!(
                    "Hi, From Main Child More {}",
                    borrow.state.to_string()
                )),
                ..Default::default()
            },
        );
        let html = rustact::html(
            "<h5><span><span><p></p></span></span><h1><h2></h2><h3><h4></h4></h3></h1></h5>"
                .to_owned(),
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
                text: Some(format!("Hi, From Main Child Extra {:?}", borrow.props)),
                ..Default::default()
            },
        );

        let vec_element = rustact::create_element(
            "div".to_owned(),
            rustact::Props {
                on_click: Some(Box::new(closure.clone())),
                class_name: Some("main-child".to_owned()),
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
                children: Some(vec![main_text, more_text, vec_element, extra_text, html]),
                ..Default::default()
            },
        );

        main
    }
}
