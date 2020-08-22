use crate::element::Element;

/// ```Components``` are the basic building blocks of the UI which needs to manage state. Each ```Component```
/// chooses how to display itself using received props and self-managed state.
/// Inspired by [Yew](https://github.com/yewstack/yew)'s [Component](https://docs.rs/yew/0.17.3/yew/html/trait.Component.html)
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

    /// Called by a ```Component's``` parent in order to pass properties.
    fn add_props(&mut self, props: Self::Properties);
}
