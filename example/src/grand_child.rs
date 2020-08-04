use crate::action::Action;
use crate::handle;
use crate::text_wrapper::text_wrapper;
use rust_fel;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct ChildProps {
    pub string_props: String,
}

#[derive(Debug, Default, Clone)]
pub struct GrandChild {
    state: i32,
    props: ChildProps,
    id: String,
}

impl GrandChild {
    pub fn create() -> handle::Handle<Self> {
        let grand_child = GrandChild {
            id: "grand-child".to_owned(),
            ..Default::default()
        };
        handle::Handle(Rc::new(RefCell::new(grand_child)))
    }
}

impl rust_fel::Component for handle::Handle<GrandChild> {
    type Properties = ChildProps;
    type Message = Action;
    type State = i32;

    fn add_props(&mut self, props: Self::Properties) {
        self.0.borrow_mut().props = props;
    }

    fn reduce_state(&mut self, message: Action) {
        match message {
            Action::Increment => self.0.borrow_mut().state += 1000,
            Action::Decrement => self.0.borrow_mut().state -= 1000,
        }

        rust_fel::re_render(self.render(), Some(self.0.borrow().id.clone()));
    }

    fn render(&self) -> rust_fel::Element {
        let mut clone = self.clone();
        let borrow = self.0.borrow();

        let grand_text = rust_fel::create_element(
            "TEXT_ELEMENT".to_owned(),
            rust_fel::Props {
                text: Some(format!("Hi, From grand Child {}", borrow.state.to_string())),
                ..Default::default()
            },
        );

        let grand_el = text_wrapper(
            "div".to_owned(),
            Some(vec![grand_text]),
            None,
            Some("main-text".to_owned()),
        );

        let more_text = rust_fel::create_element(
            "TEXT_ELEMENT".to_owned(),
            rust_fel::Props {
                text: Some(format!(
                    "Hi, From grand Child More {}",
                    borrow.state.to_string()
                )),
                ..Default::default()
            },
        );
        let anchor = rust_fel::create_element(
            "a".to_owned(),
            rust_fel::Props {
                href: Some("https://www.google.com".to_owned()),
                children: Some(vec![rust_fel::html(
                    "<span |class=anchor|>Anchor</span>".to_owned(),
                )]),
                ..Default::default()
            },
        );

        let more_el = text_wrapper(
            "div".to_owned(),
            Some(vec![more_text]),
            None,
            Some("main-text".to_owned()),
        );

        let closure = move || clone.reduce_state(Action::Decrement);

        let extra_text = rust_fel::create_element(
            "TEXT_ELEMENT".to_owned(),
            rust_fel::Props {
                text: Some(format!("Hi, From grand Child Extra {:?}", borrow.props)),
                ..Default::default()
            },
        );

        let extra_el = text_wrapper(
            "div".to_owned(),
            Some(vec![extra_text]),
            None,
            Some("main-text".to_owned()),
        );

        let html = rust_fel::html(
            "<h5 |class=grandchild-html|>
              <span |class=grandchild-html|>
                <span |class=grandchild-html|>
                  <p |class=grandchild-html|>From a P</p>
                </span>
              </span>
              <h1 |class=grandchild-html|>From an h1</h1>
              <h1 |class=grandchild-html|>
                <h2 |class=grandchild-html|>Are we parsing yet?</h2>
                <h3 |class=grandchild-html|>
                  <a |class=grandchild-html href=https://www.google.com |>Last Little Guy Here</a>
                </h3>
              </h1>
            </h5>"
                .to_owned(),
        );

        let grand = rust_fel::create_element(
            "div".to_owned(),
            rust_fel::Props {
                id: Some(self.0.borrow().id.clone()),
                mouse: Some(Box::new(closure.clone())),
                class_name: Some("grand-child".to_owned()),
                children: Some(vec![grand_el, more_el, extra_el, html, anchor]),
                ..Default::default()
            },
        );

        grand
    }
}
