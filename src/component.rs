use crate::element::Element;

pub trait Component: Sized + 'static {
  type Message: 'static;
  type Properties: 'static;
  type State: 'static;

  fn render(&self) -> Element;
  fn set_state(&mut self, state: Self::State);
  fn add_props(&mut self, props: Self::Properties);
  // fn receive_update(&mut self, message: Self::Message);
}
