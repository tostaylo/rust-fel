use crate::props::Props;
use std::fmt;

/// The structure which represents a Virtual Dom for the rust_fel library.
/// It holds Props, which in turn holds a vector of Element's as children.
/// This means ```Element``` can represent a tree of DOM elements.
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
