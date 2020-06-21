use crate::rustact;
pub fn list_item(props: rustact::Props) -> rustact::Element {
    rustact::create_element("li".to_owned(), props)
}
