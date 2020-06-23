use crate::rustact;

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

    pub fn set_state(&mut self, new_count: i32) {
        // self.count += new_count;
    }
}

impl rustact::Render for AComponent {
    fn render(&self) -> rustact::Element {
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
                // id: Some(self.id.clone()),
                children: Some(vec![hi_text]),
                // on_click: Some(Box::new(move || self.set_state(1))),
                class_name: self.class_name_from_parent.clone(),
                ..Default::default()
            },
        );

        div
    }
}
