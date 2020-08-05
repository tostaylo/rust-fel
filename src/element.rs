use crate::props::Props;
use std::fmt;

#[derive(Default)]
pub struct Element {
    pub html_type: String,
    pub props: Props,
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:#?}, {:#?} this is a element",
            self.html_type, self.props
        )
    }
}

impl Element {
    pub fn new(html_type: String, props: Props) -> Element {
        Element { html_type, props }
    }
}
