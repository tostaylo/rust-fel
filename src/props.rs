use crate::element::Element;
use std::fmt;

pub type ClosureProp = Box<dyn FnMut() -> ()>;

pub struct Props {
    pub children: Option<Vec<Element>>,
    pub text: Option<String>,
    pub on_click: Option<ClosureProp>,
    pub mouse: Option<ClosureProp>,
    pub class_name: Option<String>,
    pub id: Option<String>,
}

impl fmt::Debug for Props {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?} this is props", self.children)
    }
}

impl Default for Props {
    fn default() -> Self {
        Props {
            children: None,
            text: None,
            on_click: None,
            class_name: None,
            id: None,
            mouse: None,
        }
    }
}
