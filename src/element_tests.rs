use super::Element;
use crate::props::Props;

#[test]
fn new_preserves_html_type_and_props() {
    let element = Element::new(
        "section".to_owned(),
        Props {
            id: Some("root".to_owned()),
            text: Some("hello".to_owned()),
            ..Default::default()
        },
    );

    assert_eq!(element.html_type, "section");
    assert_eq!(element.props.id.as_deref(), Some("root"));
    assert_eq!(element.props.text.as_deref(), Some("hello"));
}

#[test]
fn debug_output_mentions_html_type() {
    let element = Element::new("article".to_owned(), Props::default());
    let debug = format!("{element:?}");

    assert!(debug.contains("article"));
    assert!(debug.contains("props"));
}
