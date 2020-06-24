use super::app;
use crate::log;
use crate::rustact;
use crate::rustact::Render;

#[derive(Debug, Default, Clone)]
pub struct AComponent {
    id: String,
    count: i32,
    class_name_from_parent: Option<String>,
}

impl AComponent {
    pub fn new(class_name_from_parent: Option<String>) -> Self {
        Self {
            id: "a_component".to_owned(),
            count: 0,
            class_name_from_parent,
        }
    }
    // TODO: How to make set_state call rustact::re_render automatically
    pub fn set_state(&mut self, new_count: i32) {
        self.count += new_count;

        rustact::re_render(self.render(), Some(self.id.clone()));
    }
}

impl rustact::Render for AComponent {
    fn render(&self) -> rustact::Element {
        let mut clone = self.clone();
        let hi_text = rustact::create_element(
            "TEXT_ELEMENT".to_owned(),
            rustact::Props {
                text: Some(format!("Hi, I am a component {}", self.count.to_string())),
                ..Default::default()
            },
        );

        let div = rustact::create_element(
            "div".to_owned(),
            rustact::Props {
                id: Some(self.id.clone()),
                children: Some(vec![hi_text]),
                class_name: self.class_name_from_parent.clone(),
                on_click: Some(Box::new(move || clone.set_state(2))),
                ..Default::default()
            },
        );
        log(&format!("{:#?} inside create", self.count));
        div
    }
}
