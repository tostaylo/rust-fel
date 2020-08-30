use crate::props::Props;
use std::fmt;

/// The structure which represents a Virtual [DOM](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Introduction) for the ```rust_fel``` library.
/// It holds [rust_fel::Props](../rust_fel/struct.Props.html) , which in turn hold a [Vec](https://doc.rust-lang.org/beta/std/vec/) of ```Element's``` as ```children```.
/// This means ```Element``` can represent a tree of [DOM Elements](https://developer.mozilla.org/en-US/docs/Web/API/Element).
#[derive(Default)]
pub struct Element {
    pub html_type: String,
    pub props: Props,
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:#?} this is a element html_type and here are it's props -> {:#?}",
            self.html_type, self.props
        )
    }
}

impl Element {
    pub fn new(html_type: String, props: Props) -> Element {
        Element { html_type, props }
    }
}
