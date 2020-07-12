use crate::main_child::MainChild;
use crate::rustact;
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, Default, Clone)]
pub struct Main {
    child: rustact::Handle<MainChild>,
    id: String,
    state: i32,
    props: String,
}

impl Main {
    pub fn create() -> rustact::Handle<Self> {
        let main = Main {
            id: "main".to_owned(),
            state: 2,
            child: MainChild::create(),
            ..Default::default()
        };
        rustact::Handle(Rc::new(RefCell::new(main)))
    }
}

impl rustact::Component for rustact::Handle<Main> {
    type Properties = String;
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
        let mut borrow = self.0.borrow_mut();
        let state = borrow.state;
        borrow.child.add_props(state.to_string());

        let main_text = rustact::create_element(
            "TEXT_ELEMENT".to_owned(),
            rustact::Props {
                text: Some(format!("Hi, From Main {}", state.to_string())),
                ..Default::default()
            },
        );

        let more_text = rustact::create_element(
            "TEXT_ELEMENT".to_owned(),
            rustact::Props {
                text: Some(format!("Hi, From More {}", state.to_string())),
                ..Default::default()
            },
        );
        let html = rustact::html(
            "<h5><span><span><p></p></span></span><h1><h2></h2><h3><h4></h4></h3></h1></h5>"
                .to_owned(),
        );

        let main = rustact::create_element(
            "div".to_owned(),
            rustact::Props {
                id: Some(borrow.id.clone()),
                mouse: Some(Box::new(move || clone.set_state(2))),
                class_name: Some("main".to_owned()),
                children: Some(vec![main_text, more_text, html, borrow.child.render()]),
                ..Default::default()
            },
        );

        main
    }
}
