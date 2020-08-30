use crate::element::Element;

/// ```Components``` are the basic building blocks of the UI. Each ```Component```
/// chooses how to display itself using received props and self-managed state.
/// Inspired by [Yew](https://github.com/yewstack/yew)'s [Component](https://docs.rs/yew/0.17.3/yew/html/trait.Component.html)
/// # Examples
/// A rust_fel [struct](https://doc.rust-lang.org/std/keyword.struct.html) component implements [rust_fel::Component](../rust_fel/trait.Component.html)
///```ignore
///use crate::action::Action;
///use crate::handle;
///use crate::main_child::{ChildProps, MainChild};
///use std::cell::RefCell;
///use std::rc::Rc;
///use wasm_bindgen::JsCast;
///
///#[derive(Debug, Default, Clone)]
///pub struct MainState {
///    count: i32,
///}
///
///pub enum Actions {
///    Counter(Action),
///}
///
///#[derive(Debug, Default, Clone)]
///pub struct Main {
///    child: handle::Handle<MainChild>,
///    id: String,
///    state: MainState,
///    props: String,
///}
///
///impl Main {
///    pub fn create() -> handle::Handle<Self> {
///        let main = Main {
///            id: "main".to_owned(),
///            state: MainState {
///                count: 0,
///            },
///            child: MainChild::create(),
///            ..Default::default()
///        };
///        handle::Handle(Rc::new(RefCell::new(main)))
///    }
///}
///
///impl rust_fel::Component for handle::Handle<Main> {
///    type Properties = String;
///    type Message = Actions;
///    type State = MainState;
///
///    fn add_props(&mut self, props: Self::Properties) {
///        self.0.borrow_mut().props = props;
///    }
///
///    fn reduce_state(&mut self, message: Actions) {
///        match message {
///            Actions::Counter(Action::Increment) => self.0.borrow_mut().state.count += 100,
///            Actions::Counter(Action::Decrement) => self.0.borrow_mut().state.count -= 100,
///        }
///
///        rust_fel::re_render(self.render(), Some(self.0.borrow().id.clone()));
///    }
///
///    fn render(&self) -> rust_fel::Element {
///        let mut clone_for_props_closure = self.clone();
///        let mut clone_for_inc = self.clone();
///        let mut borrow = self.0.borrow_mut();
///        let state = borrow.state.clone();
///        let props_closure = Rc::new(RefCell::new(move || {
///            clone_for_props_closure.reduce_state(Actions::Counter(Action::Decrement))
///        }));
///
///        let child_props = ChildProps {
///            counter_props: state.count.to_string(),
///            closure: Some(props_closure),
///        };
///
///        borrow.child.add_props(child_props);
///
///        let main_text = rust_fel::html(format!(
///            "<span | data-cy=main-text| >Main {}</span>",
///            state.count.to_string()
///        ));
///
///        let inc_button_text = rust_fel::Element::new(
///            "TEXT_ELEMENT".to_owned(),
///            rust_fel::Props {
///                text: Some("Increment".to_owned()),
///                ..Default::default()
///            },
///        );
///
///        let inc_button = rust_fel::Element::new(
///            "button".to_owned(),
///            rust_fel::Props {
///                on_click: Some(Box::new(move || {
///                    clone_for_inc.reduce_state(Actions::Counter(Action::Increment))
///                })),
///                data_cy: Some("increment-main".to_owned()),
///                children: Some(vec![inc_button_text]),
///                ..Default::default()
///            },
///        );
///
///        let main_el = rust_fel::Element::new(
///            "div".to_owned(),
///            rust_fel::Props {
///                class_name: Some("main-el".to_owned()),
///                children: Some(vec![main_text, inc_button, input_wrapper]),
///                ..Default::default()
///            },
///        );
///
///        let child_wrapper = rust_fel::Element::new(
///            "div".to_owned(),
///            rust_fel::Props {
///                class_name: Some("child-wrapper".to_owned()),
///                children: Some(vec![borrow.child.render()]),
///                ..Default::default()
///            },
///        );
///
///        rust_fel::Element::new(
///            "div".to_owned(),
///            rust_fel::Props {
///                id: Some(borrow.id.clone()),
///                class_name: Some("main".to_owned()),
///                children: Some(vec![main_el, child_wrapper]),
///                ..Default::default()
///            },
///        )
///    }
///}
///
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

    /// Called by a ```Component's``` parent in order to pass properties.
    fn add_props(&mut self, props: Self::Properties);
}
