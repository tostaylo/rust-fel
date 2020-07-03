use super::app;
use crate::log;
use crate::rustact;
use crate::rustact::Render;
use crate::rustact::SetState;
use crate::use_component_child::use_component_child;

pub fn use_component() -> rustact::Component<String> {
    let component: rustact::Component<String> = rustact::Component::new(
        "use-component".to_owned(),
        Some(vec![use_component_child()]),
        "hi".to_owned(),
    );
    impl rustact::SetState<String> for rustact::Component<String> {
        fn set_state(&mut self, new_state: String) {
            self.state = new_state;
            rustact::re_render(self.render(), Some(self.id.clone()));
        }
    }

    impl rustact::Render for rustact::Component<String> {
        fn render(&self) -> rustact::Element {
            let mut clone = self.clone();

            let div = rustact::create_element(
                "div".to_owned(),
                rustact::Props {
                    id: Some(self.id.clone()),
                    // children: Some(vec![.render()]),
                    on_click: Some(Box::new(move || clone.set_state("hohohoho".to_owned()))),
                    ..Default::default()
                },
            );
            log(&format!("{:#?} inside use compoennt", self.state));
            div
        }
    }
    component
}
