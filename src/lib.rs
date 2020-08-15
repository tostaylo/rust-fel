//! A Rust Front End Library

pub mod app;
pub mod component;
pub mod element;
pub mod js;
pub mod props;
pub mod render;
pub mod rsx;
pub mod wrapper;

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
#[doc(inline)]
pub use crate::wrapper::wrapper;
