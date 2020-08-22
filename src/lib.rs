//! A Rust Front End Library.  
//!
//! Experimental.  
//!
//! Relies on rust-wasm.  
//!
//! Very lightweight and does not support much of the HTML spec. More work needs to be done to truly make this a viable option
//! for creating client side front end's with rust-wasm.

#![allow(clippy::single_match)]

pub mod app;
pub mod component;
pub mod element;
pub mod js;
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
pub use crate::js::log;
#[doc(inline)]
pub use crate::props::{ClosureProp, Props};
#[doc(inline)]
pub use crate::render::{re_render, render};
#[doc(inline)]
pub use crate::rsx::html;
