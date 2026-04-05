use super::App;
use crate::component::Component;
use crate::element::Element;
use crate::props::Props;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Debug)]
struct DummyComponent {
    rendered: Rc<Cell<bool>>,
}

impl Component for DummyComponent {
    type Properties = ();
    type Message = ();
    type State = ();

    fn render(&self) -> Element {
        self.rendered.set(true);
        Element::new("div".to_owned(), Props::default())
    }

    fn reduce_state(&mut self, _message: Self::Message) {}

    fn add_props(&mut self, _props: Self::Properties) {}
}

#[test]
fn new_stores_component_without_rendering() {
    let rendered = Rc::new(Cell::new(false));
    let app = App::new(DummyComponent {
        rendered: Rc::clone(&rendered),
    });

    assert!(!rendered.get());
    assert!(!app.component.rendered.get());
}
