use crate::element::Element;

/// ```Components``` are the basic building blocks of the UI. Each ```Component```
/// chooses how to display itself using received props and self-managed state.
/// Inspired by [Yew](https://github.com/yewstack/yew)'s [Component](https://docs.rs/yew/0.17.3/yew/html/trait.Component.html)
/// # Examples
/// A rust_fel [struct](https://doc.rust-lang.org/std/keyword.struct.html) component implements [rust_fel::Component](../rust_fel/trait.Component.html)
///```
///use rust_fel::{Component, Element, Props};
///
///#[derive(Default)]
///struct Counter {
///    count: i32,
///}
///
///enum Message {
///    Increment,
///    Decrement,
///}
///
///impl Component for Counter {
///    type Properties = String;
///    type Message = Message;
///    type State = i32;
///
///    fn add_props(&mut self, props: Self::Properties) {
///        self.count = props.parse().unwrap();
///    }
///
///    fn reduce_state(&mut self, message: Self::Message) {
///        match message {
///            Message::Increment => self.count += 1,
///            Message::Decrement => self.count -= 1,
///        }
///    }
///
///    fn render(&self) -> Element {
///        Element::new(
///            "div".to_owned(),
///            Props {
///                text: Some(self.count.to_string()),
///                ..Default::default()
///            },
///        )
///    }
///}
///
///let mut counter = Counter::default();
///counter.add_props("41".to_owned());
///counter.reduce_state(Message::Increment);
///let view = counter.render();
///
///assert_eq!(view.html_type, "div");
///assert_eq!(view.props.text.as_deref(), Some("42"));
///```
pub trait Component: Sized + 'static {
    /// Messages are used to make ```Components``` dynamic and interactive.
    type Message: 'static;

    /// ```Properties``` are the inputs to a ```Component``` and should not be mutated.
    type Properties: 'static;

    /// A ```Component's``` internal state.
    type State: 'static;

    /// Construct your html view here.
    fn render(&self) -> Element;

    /// How a ```Component manages``` internal state.
    fn reduce_state(&mut self, message: Self::Message);

    /// Invoked by a ```Component's``` parent in order to pass properties.
    fn add_props(&mut self, props: Self::Properties);
}
