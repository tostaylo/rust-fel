//! A Rust Front-End Library.  
//!
//! Experimental.  
//!
//! Relies on [rustwasm](https://github.com/rustwasm).  
//!
//! Very lightweight and does not support much of the [HTML Standard](https://html.spec.whatwg.org/). More work needs to be done to truly make this a viable option
//! for creating client side front-ends with [rustwasm](https://github.com/rustwasm).  
//! ### Features
//! * State Management
//! * [JSX](https://github.com/facebook/jsx) -like syntax
//! * [DOM](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Introduction) construction from a Virtual [DOM](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Introduction).
//!
//! # Use
//! ```no_run
//! use rust_fel::{App, Component, Element, Props};
//!
//! #[derive(Debug)]
//! struct Main;
//!
//! impl Component for Main {
//!     type Properties = ();
//!     type Message = ();
//!     type State = ();
//!
//!     fn render(&self) -> Element {
//!         Element::new("div".to_owned(), Props::default())
//!     }
//!
//!     fn reduce_state(&mut self, _message: Self::Message) {}
//!
//!     fn add_props(&mut self, _props: Self::Properties) {}
//! }
//!
//! let app = App::new(Main);
//! app.mount("root");
//! ```
//! # Examples
//! A ```rust_fel``` [struct](https://doc.rust-lang.org/std/keyword.struct.html) component implements [rust_fel::Component](../rust_fel/trait.Component.html)
//!```
//!use rust_fel::{Component, Element, Props};
//!
//!#[derive(Default)]
//!struct Counter {
//!    count: i32,
//!}
//!
//!enum Message {
//!    Increment,
//!    Decrement,
//!}
//!
//!impl Component for Counter {
//!    type Properties = String;
//!    type Message = Message;
//!    type State = i32;
//!
//!    fn add_props(&mut self, props: Self::Properties) {
//!        self.count = props.parse().unwrap();
//!    }
//!
//!    fn reduce_state(&mut self, message: Self::Message) {
//!        match message {
//!            Message::Increment => self.count += 1,
//!            Message::Decrement => self.count -= 1,
//!        }
//!    }
//!
//!    fn render(&self) -> Element {
//!        Element::new(
//!            "div".to_owned(),
//!            Props {
//!                text: Some(self.count.to_string()),
//!                ..Default::default()
//!            },
//!        )
//!    }
//!}
//!
//!let mut counter = Counter::default();
//!counter.add_props("41".to_owned());
//!counter.reduce_state(Message::Increment);
//!let view = counter.render();
//!
//!assert_eq!(view.html_type, "div");
//!assert_eq!(view.props.text.as_deref(), Some("42"));
//!```
//!  A ```rust_fel``` functional component with [rust_fel::html](../rust_fel/rsx/fn.html.html)
//!```
//!pub fn theme_switcher(on_click: rust_fel::ClosureProp, title: String) -> rust_fel::Element {
//!    let text = rust_fel::html(format!(
//!        "<span |class=theme-switcher-text|>{}</span>",
//!        title
//!    ));
//!
//!    let theme_button = rust_fel::Element::new(
//!        "button".to_owned(),
//!        {
//!            let mut props = rust_fel::Props {
//!                on_click: Some(on_click),
//!                children: Some(vec![text]),
//!                ..Default::default()
//!            };
//!            props.set_attribute("type", "button");
//!            props.set_attribute("class", "theme-switcher-button");
//!            props.set_attribute("data-cy", title);
//!            props
//!        },
//!    );
//!
//!    rust_fel::Element::new(
//!        "li".to_owned(),
//!        rust_fel::Props {
//!            children: Some(vec![theme_button]),
//!            ..Default::default()
//!        },
//!    )
//!}
//!
//!let _theme = theme_switcher(Box::new(|| {}), "Light".to_owned());
//!```

#![doc(html_root_url = "https://docs.rs/rust-fel/0.1.2")] // Must be kept in sync with Cargo.toml
#![allow(clippy::single_match)]
/// Module containing the [rust_fel::App](../rust_fel/struct.App.html) [struct](https://doc.rust-lang.org/std/keyword.struct.html) which mounts your ```App``` to the [DOM](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Introduction).
pub mod app;
/// Module containing the [rust_fel::Component](../rust_fel/trait.Component.html) trait. Necessary for state management at the [struct](https://doc.rust-lang.org/std/keyword.struct.html) level.
pub mod component;
/// Module containing the [rust_fel::Element](../rust_fel/struct.Element.html) [struct](https://doc.rust-lang.org/std/keyword.struct.html) which acts as a Virtual [DOM](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Introduction).
pub mod element;
/// Module containing the [rust_fel::Props](../rust_fel/struct.Props.html) [struct](https://doc.rust-lang.org/std/keyword.struct.html) which allows an [rust_fel::Element](../rust_fel/struct.Element.html) to have ```properties``` and ```children```.
pub mod props;
/// Module containing the functions to ```render``` and [rust_fel::re_render](../rust_fel/fn.re_render.html) the [rust_fel::App](../rust_fel/struct.App.html).
pub mod render;
/// Module containing all the functions needed for the [rust_fel::html](../rust_fel/fn.html.html) function to create [rust_fel::Element](../rust_fel/struct.Element.html) from strings of [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML).
pub mod rsx;

#[doc(inline)]
pub use crate::app::App;
#[doc(inline)]
pub use crate::component::Component;
#[doc(inline)]
pub use crate::element::Element;
#[doc(inline)]
pub use crate::props::{ClosureProp, Props};
#[doc(inline)]
pub use crate::render::re_render;
#[doc(inline)]
pub use crate::rsx::html;
