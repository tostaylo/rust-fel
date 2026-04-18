use crate::element::Element;
use std::fmt;

/// A type commonly used for construction of a [wasm-bindgen Closure](https://docs.rs/wasm-bindgen/0.2.67/wasm_bindgen/closure/struct.Closure.html) for use with [DOM Element Event Handlers](https://developer.mozilla.org/en-US/docs/Web/Guide/Events/Event_handlers).
/// # Examples
/// In this example ```&self``` is a [tuple struct](https://doc.rust-lang.org/1.9.0/book/structs.html#tuple-structs) which implements [rust_fel::Component](../rust_fel/trait.Component.html)
///```
///use rust_fel::{ClosureProp, Element, Props};
///
///let on_click: ClosureProp = Box::new(|| {});
///let element = Element::new(
///    "button".to_owned(),
///    Props {
///        on_click: Some(on_click),
///        text: Some("Toggle theme".to_owned()),
///        ..Default::default()
///    },
///);
///
///assert_eq!(element.html_type, "button");
///assert!(element.props.on_click.is_some());
///```
pub type ClosureProp = Box<dyn FnMut()>;

/// A [struct](https://doc.rust-lang.org/std/keyword.struct.html) holding attributes for a Virtual [DOM](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Introduction) [rust_fel::Element](../rust_fel/struct.Element.html).  
/// ```Elements``` can have ```children``` stored inside a [std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html).
#[derive(Default)]
pub struct Props {
    pub children: Option<Vec<Element>>,
    pub text: Option<String>,
    pub on_click: Option<ClosureProp>,
    pub mouse: Option<ClosureProp>,
    pub attributes: Option<Vec<(String, String)>>,
}

impl fmt::Debug for Props {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:#?} props.children, {:#?} props.text, {:#?} props.attributes",
            self.children, self.text, self.attributes
        )
    }
}

impl Props {
    pub fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes.as_ref().and_then(|attributes| {
            attributes
                .iter()
                .find(|(attribute_name, _)| attribute_name == name)
                .map(|(_, value)| value.as_str())
        })
    }

    pub fn set_attribute(&mut self, name: impl Into<String>, value: impl Into<String>) {
        let name = name.into();
        let value = value.into();
        let attributes = self.attributes.get_or_insert_with(Vec::new);

        if let Some((_, existing_value)) = attributes
            .iter_mut()
            .find(|(attribute_name, _)| attribute_name == &name)
        {
            *existing_value = value;
            return;
        }

        attributes.push((name, value));
    }
}

#[cfg(test)]
#[path = "props_tests.rs"]
mod props_tests;
