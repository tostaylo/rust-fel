use super::app;
use crate::log;
use crate::rustact;
use crate::rustact::Render;
use crate::rustact::SetState;

pub fn use_component_child() -> rustact::Component<i32> {
    let component = rustact::Component::new("use-component-child".to_owned(), None, 56);
    impl rustact::SetState<i32> for rustact::Component<i32> {
        fn set_state(&mut self, new_state: i32) {
            self.state = new_state;
            rustact::re_render(self.render(), Some(self.id.clone()));
        }
    }

    impl rustact::Render for rustact::Component<i32> {
        fn render(&self) -> rustact::Element {
            let mut clone = self.clone();

            let div = rustact::create_element(
                "div".to_owned(),
                rustact::Props {
                    id: Some(self.id.clone()),
                    on_click: Some(Box::new(move || clone.set_state(44))),
                    ..Default::default()
                },
            );
            log(&format!(
                "{:#?} inside use component child child compoennt",
                self.state
            ));
            div
        }
    }
    component
}
