use super::Element;
use crate::props::Props;

#[test]
fn new_preserves_html_type_and_props() {
    let mut props = Props {
        text: Some("hello".to_owned()),
        ..Default::default()
    };
    props.set_attribute("id", "root");

    let element = Element::new("section".to_owned(), props);

    assert_eq!(element.html_type, "section");
    assert_eq!(element.props.attribute("id"), Some("root"));
    assert_eq!(element.props.text.as_deref(), Some("hello"));
}

#[test]
fn debug_output_mentions_html_type() {
    let element = Element::new("article".to_owned(), Props::default());
    let debug = format!("{element:?}");

    assert!(debug.contains("article"));
    assert!(debug.contains("props"));
}
