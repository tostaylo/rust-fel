use crate::element::Element;

pub trait Component: Sized + 'static {
    type Message: 'static;
    type Properties: 'static;
    type State: 'static;

    fn render(&self) -> Element;
    fn reduce_state(&mut self, message: Self::Message);
    fn add_props(&mut self, props: Self::Properties);
}
