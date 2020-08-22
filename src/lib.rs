//! A Rust Front-End Library.  
//!
//! Experimental.  
//!
//! Relies on [rustwasm](https://github.com/rustwasm).  
//!
//! Very lightweight and does not support much of the HTML spec. More work needs to be done to truly make this a viable option
//! for creating client side front-ends with [rustwasm](https://github.com/rustwasm).  
//! ### Features
//! * State Management
//! * JSX-like syntax
//! * DOM construction from a Virtual DOM
//!
//! # Use
//! ```ignore
//!
//! use crate::main_component::Main;
//! use wasm_bindgen::prelude::*;
//! extern crate rust_fel;
//!
//!
//! // Called when the wasm module is instantiated
//! #[wasm_bindgen(start)]
//! pub fn main() -> Result<(), JsValue> {
//!     let main = Main::create();
//!     let app = rust_fel::App::new(main);
//!     app.mount("root");
//!
//!     Ok(())
//!
//! ```
//! # Examples
//! A rust_fel struct component implements [rust_fel::Component](../rust_fel/trait.Component.html)
//!```ignore
//!use crate::action::Action;
//!use crate::handle;
//!use crate::main_child::{ChildProps, MainChild};
//!use std::cell::RefCell;
//!use std::rc::Rc;
//!use wasm_bindgen::JsCast;
//!
//!#[derive(Debug, Default, Clone)]
//!pub struct MainState {
//!    count: i32,
//!}
//!
//!pub enum Actions {
//!    Counter(Action),
//!}
//!
//!#[derive(Debug, Default, Clone)]
//!pub struct Main {
//!    child: handle::Handle<MainChild>,
//!    id: String,
//!    state: MainState,
//!    props: String,
//!}
//!
//!impl Main {
//!    pub fn create() -> handle::Handle<Self> {
//!        let main = Main {
//!            id: "main".to_owned(),
//!            state: MainState {
//!                count: 0,
//!            },
//!            child: MainChild::create(),
//!            ..Default::default()
//!        };
//!        handle::Handle(Rc::new(RefCell::new(main)))
//!    }
//!}
//!
//!impl rust_fel::Component for handle::Handle<Main> {
//!    type Properties = String;
//!    type Message = Actions;
//!    type State = MainState;
//!
//!    fn add_props(&mut self, props: Self::Properties) {
//!        self.0.borrow_mut().props = props;
//!    }
//!
//!    fn reduce_state(&mut self, message: Actions) {
//!        match message {
//!            Actions::Counter(Action::Increment) => self.0.borrow_mut().state.count += 100,
//!            Actions::Counter(Action::Decrement) => self.0.borrow_mut().state.count -= 100,
//!        }
//!
//!        rust_fel::re_render(self.render(), Some(self.0.borrow().id.clone()));
//!    }
//!
//!    fn render(&self) -> rust_fel::Element {
//!        let mut clone_for_props_closure = self.clone();
//!        let mut clone_for_inc = self.clone();
//!        let mut borrow = self.0.borrow_mut();
//!        let state = borrow.state.clone();
//!        let props_closure = Rc::new(RefCell::new(move || {
//!            clone_for_props_closure.reduce_state(Actions::Counter(Action::Decrement))
//!        }));
//!
//!        let child_props = ChildProps {
//!            counter_props: state.count.to_string(),
//!            closure: Some(props_closure),
//!        };
//!
//!        borrow.child.add_props(child_props);
//!
//!        let main_text = rust_fel::html(format!(
//!            "<span | data-cy=main-text| >Main {}</span>",
//!            state.count.to_string()
//!        ));
//!
//!        let inc_button_text = rust_fel::Element::new(
//!            "TEXT_ELEMENT".to_owned(),
//!            rust_fel::Props {
//!                text: Some("Increment".to_owned()),
//!                ..Default::default()
//!            },
//!        );
//!
//!        let inc_button = rust_fel::Element::new(
//!            "button".to_owned(),
//!            rust_fel::Props {
//!                on_click: Some(Box::new(move || {
//!                    clone_for_inc.reduce_state(Actions::Counter(Action::Increment))
//!                })),
//!                data_cy: Some("increment-main".to_owned()),
//!                children: Some(vec![inc_button_text]),
//!                ..Default::default()
//!            },
//!        );
//!
//!        let main_el = rust_fel::Element::new(
//!            "div".to_owned(),
//!            rust_fel::Props {
//!                class_name: Some("main-el".to_owned()),
//!                children: Some(vec![main_text, inc_button, input_wrapper]),
//!                ..Default::default()
//!            },
//!        );
//!
//!        let child_wrapper = rust_fel::Element::new(
//!            "div".to_owned(),
//!            rust_fel::Props {
//!                class_name: Some("child-wrapper".to_owned()),
//!                children: Some(vec![borrow.child.render()]),
//!                ..Default::default()
//!            },
//!        );
//!
//!        rust_fel::Element::new(
//!            "div".to_owned(),
//!            rust_fel::Props {
//!                id: Some(borrow.id.clone()),
//!                class_name: Some("main".to_owned()),
//!                children: Some(vec![main_el, child_wrapper]),
//!                ..Default::default()
//!            },
//!        )
//!    }
//!}
//!
//!```
//!
//!  A rust_fel functional component with [rust_fel::html](../rust_fel/rsx/fn.html.html)
//!```ignore
//!pub fn theme_switcher(on_click: rust_fel::ClosureProp, title: String) -> rust_fel::Element {
//!    let text = rust_fel::html(format!(
//!        "<span |class=theme-switcher-text|>{}</span>",
//!        title
//!    ));
//!
//!    let theme_button = rust_fel::Element::new(
//!        "button".to_owned(),
//!        rust_fel::Props {
//!            on_click: Some(on_click),
//!            type_attr: Some("button".to_owned()),
//!            class_name: Some("theme-switcher-button".to_owned()),
//!            children: Some(vec![text]),
//!            data_cy: Some(title),
//!            ..Default::default()
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
//!```

#![allow(clippy::single_match)]

pub mod app;
pub mod component;
pub mod element;
pub mod props;
pub mod render;
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
pub use crate::render::{re_render, render};
#[doc(inline)]
pub use crate::rsx::html;
