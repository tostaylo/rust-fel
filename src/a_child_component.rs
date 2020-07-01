use super::app;
use crate::log;
use crate::rustact;
use crate::rustact::Render;

#[derive(Debug, Default, Clone)]
pub struct AChildComponent {
    id: String,
    count: i32,
}

impl AChildComponent {
    pub fn new() -> Self {
        Self {
            id: "a_child_component".to_owned(),
            count: 0,
        }
    }
    // TODO: How to make set_state call rustact::re_render automatically
    pub fn set_state(&mut self, new_count: i32) {
        self.count += new_count;

        rustact::re_render(self.render(), Some(self.id.clone()));
    }
}

impl rustact::Render for AChildComponent {
    fn render(&self) -> rustact::Element {
        let mut clone = self.clone();
        let hi_text = rustact::create_element(
            "TEXT_ELEMENT".to_owned(),
            rustact::Props {
                text: Some(format!(
                    "Hi, I am a child component {}",
                    self.count.to_string()
                )),
                ..Default::default()
            },
        );

        let div = rustact::create_element(
            "div".to_owned(),
            rustact::Props {
                id: Some(self.id.clone()),
                children: Some(vec![hi_text]),
                on_click: Some(Box::new(move || clone.set_state(2))),
                ..Default::default()
            },
        );
        log(&format!("{:#?} inside child compoennt", self.count));
        div
    }
}
