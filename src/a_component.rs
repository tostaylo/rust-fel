use super::app;
use crate::a_child_component::AChildComponent;
use crate::log;
use crate::rustact;
use crate::rustact::Render;

#[derive(Default, Clone)]
pub struct AComponent {
    id: String,
    count: i32,
    class_name_from_parent: Option<String>,
    components: Vec<Box<dyn rustact::Render>>,
}

impl AComponent {
    pub fn new(class_name_from_parent: Option<String>) -> Self {
        Self {
            id: "a_component".to_owned(),
            count: 0,
            class_name_from_parent,
            components: vec![Box::new(AChildComponent::new())],
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
        // let children: Vec<rustact::Element> = self
        //     .components
        //     .iter()
        //     .map(|component| component.render())
        //     .collect();

        let mut first_child = vec![hi_text];

        let div = rustact::create_element(
            "div".to_owned(),
            rustact::Props {
                id: Some(self.id.clone()),
                children: Some(vec![self.components[0].render()]),
                class_name: self.class_name_from_parent.clone(),
                // on_click: Some(Box::new(move || clone.set_state(2))),
                ..Default::default()
            },
        );

        div
    }
}
