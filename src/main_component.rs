use crate::main_child::MainChild;
use crate::rustact;
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, Default, Clone)]
pub struct Main {
    props: i32,
    child: rustact::Handle<MainChild>,
    id: String,
    count: i32,
}

impl Main {
    pub fn create(props: i32) -> rustact::Handle<Self> {
        let main = Main {
            props,
            id: "main".to_owned(),
            child: MainChild::create(5),
            ..Default::default()
        };
        rustact::Handle(Rc::new(RefCell::new(main)))
    }
}

impl rustact::Component for rustact::Handle<Main> {
    type Properties = i32;
    type Message = String;
    type State = i32;

    fn set_state(&mut self, new_count: Self::State) {
        self.0.borrow_mut().count += new_count;
        rustact::re_render(self.render(None), Some(self.0.borrow().id.clone()));
    }

    fn render(&self, props: Option<Self::Properties>) -> rustact::Element {
        let mut clone = self.clone();
        let borrow = self.0.borrow();
        let main_text = rustact::create_element(
            "TEXT_ELEMENT".to_owned(),
            rustact::Props {
                text: Some(format!("Hi, From Main {}", borrow.count.to_string())),
                ..Default::default()
            },
        );

        let more_text = rustact::create_element(
            "TEXT_ELEMENT".to_owned(),
            rustact::Props {
                text: Some(format!("Hi, From More {}", borrow.count.to_string())),
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
                children: Some(vec![
                    main_text,
                    more_text,
                    html,
                    borrow.child.render(Some(borrow.count)),
                ]),
                ..Default::default()
            },
        );

        main
    }
}
