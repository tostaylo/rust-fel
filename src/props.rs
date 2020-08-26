use crate::element::Element;
use std::fmt;

/// A type commonly used for passing closures to DOM element event handlers
///```ignore
///  let mut new_clone = self.clone();
///  let (theme_onclick, theme_title) = match action {
///      Actions::LightMode => (
///          Box::new(move || new_clone.reduce_state(Actions::LightMode))
///              as rust_fel::ClosureProp,
///          "Light Mode".to_owned(),
///      ),
///
///      Actions::DarkMode => (
///          Box::new(move || new_clone.reduce_state(Actions::DarkMode))
///              as rust_fel::ClosureProp,
///          "Dark Mode".to_owned(),
///      ),
///      _ => (Box::new(|| ()) as rust_fel::ClosureProp, "".to_owned()),
///  };
///```
pub type ClosureProp = Box<dyn FnMut()>;

/// A struct holding attributes for a Virtual Dom [rust_fel::Element](../rust_fel/struct.Element.html).  
/// ```Elements``` can have children that are also stored here inside a [std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html).
pub struct Props {
    pub children: Option<Vec<Element>>,
    pub text: Option<String>,
    pub on_click: Option<ClosureProp>,
    pub mouse: Option<ClosureProp>,
    pub class_name: Option<String>,
    pub id: Option<String>,
    pub href: Option<String>,
    pub src: Option<String>,
    pub type_attr: Option<String>,
    pub role: Option<String>,
    pub data_cy: Option<String>,
}

impl fmt::Debug for Props {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:#?} props.children, {:#?} props.text,  {:#?} props.class_name {:#?} props.id",
            self.children, self.text, self.class_name, self.id
        )
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
            href: None,
            src: None,
            type_attr: None,
            role: None,
            data_cy: None,
        }
    }
}
