use crate::rustact;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct MainChild {
    props: i32,
    count: i32,
    id: String,
}

impl MainChild {
    pub fn create(props: i32) -> rustact::Handle<Self> {
        let main_child = MainChild {
            props,
            id: "main-child".to_owned(),
            ..Default::default()
        };
        rustact::Handle(Rc::new(RefCell::new(main_child)))
    }
}

impl rustact::Component for rustact::Handle<MainChild> {
    type Properties = i32;
    type Message = String;
    type State = i32;

    fn set_state(&mut self, new_count: Self::State) {
        self.0.borrow_mut().count += new_count;
        rustact::re_render(self.render(), Some(self.0.borrow().id.clone()));
    }

    fn render(&self) -> rustact::Element {
        let main_text = rustact::create_element(
            "TEXT_ELEMENT".to_owned(),
            rustact::Props {
                text: Some(format!(
                    "Hi, From Main Child {}",
                    self.0.borrow().count.to_string()
                )),
                ..Default::default()
            },
        );
        let html = rustact::html(
            "<h5><span><span><p></p></span></span><h1><h2></h2><h3><h4></h4></h3></h1></h5>"
                .to_owned(),
        );
        let mut clone = self.clone();
        let main = rustact::create_element(
            "div".to_owned(),
            rustact::Props {
                id: Some(self.0.borrow().id.clone()),
                on_click: Some(Box::new(move || clone.set_state(2))),
                class_name: Some("main-child".to_owned()),
                children: Some(vec![main_text, html]),
                ..Default::default()
            },
        );

        main
    }
}
