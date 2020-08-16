use crate::action::Action;
use crate::handle;
use rust_fel;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct GCProps {
    pub input_props: String,
}

#[derive(Debug, Default, Clone)]
pub struct GrandChild {
    state: i32,
    props: GCProps,
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
    type Properties = GCProps;
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

        let grand_text = rust_fel::html(format!(
            "<span | data-cy=grandchild-text| >GrandChild {}</span>",
            borrow.state.to_string()
        ));

        let props_html = rust_fel::html(format!(
            "<p><span>Text From Main: </span><span |data-cy=grandchild-props-text| >{}</span></p>",
            borrow.props.input_props
        ));

        let closure = move || clone.reduce_state(Action::Decrement);
        let dec_button_text = rust_fel::Element::new(
            "TEXT_ELEMENT".to_owned(),
            rust_fel::Props {
                text: Some("Decrement".to_owned()),
                ..Default::default()
            },
        );

        let dec_button = rust_fel::Element::new(
            "button".to_owned(),
            rust_fel::Props {
                data_cy: Some("decrement-grandchild".to_owned()),
                on_click: Some(Box::new(closure)),
                children: Some(vec![dec_button_text]),
                ..Default::default()
            },
        );

        let grand_el = rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                children: Some(vec![grand_text, dec_button, props_html]),
                class_name: Some("main-el".to_owned()),
                ..Default::default()
            },
        );
        let grand = rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                id: Some(self.0.borrow().id.clone()),
                class_name: Some("grand-child".to_owned()),
                children: Some(vec![grand_el]),
                ..Default::default()
            },
        );

        grand
    }
}
