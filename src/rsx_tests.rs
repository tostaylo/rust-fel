use super::html;

#[test]
fn html_preserves_attributes_and_nested_text() {
    let view = html(
        "<main |id=app class=page aria-live=polite|><button |type=button data-cy=increment aria-label=Add|>Add</button></main>"
            .to_owned(),
    );

    assert_eq!(view.html_type, "main");
    assert_eq!(view.props.attribute("id"), Some("app"));
    assert_eq!(view.props.attribute("class"), Some("page"));
    assert!(view
        .props
        .attributes
        .as_ref()
        .is_some_and(|attributes| attributes
            .iter()
            .any(|(name, value)| name == "aria-live" && value == "polite")));

    let children = view.props.children.expect("main should have children");
    assert_eq!(children.len(), 1);
    assert_eq!(children[0].html_type, "button");
    assert_eq!(children[0].props.attribute("type"), Some("button"));
    assert_eq!(children[0].props.attribute("data-cy"), Some("increment"));
    assert!(children[0]
        .props
        .attributes
        .as_ref()
        .is_some_and(|attributes| attributes
            .iter()
            .any(|(name, value)| name == "aria-label" && value == "Add")));

    let button_children = children[0]
        .props
        .children
        .as_ref()
        .expect("button should have a text child");
    assert_eq!(button_children.len(), 1);
    assert_eq!(button_children[0].html_type, "TEXT_ELEMENT");
    assert_eq!(button_children[0].props.text.as_deref(), Some("Add"));
}

#[test]
fn html_keeps_unknown_attributes_without_predeclaring_them() {
    let view = html(
        "<input |placeholder=Email aria-label=EmailInput autocomplete=email|></input>"
            .to_owned(),
    );

    let attributes = view
        .props
        .attributes
        .as_ref()
        .expect("input should keep parsed attributes");

    assert!(attributes
        .iter()
        .any(|(name, value)| name == "placeholder" && value == "Email"));
    assert!(attributes
        .iter()
        .any(|(name, value)| name == "aria-label" && value == "EmailInput"));
    assert!(attributes
        .iter()
        .any(|(name, value)| name == "autocomplete" && value == "email"));
}

#[test]
fn html_preserves_sibling_order() {
    let view = html("<div><span>first</span><span>second</span></div>".to_owned());
    let children = view.props.children.expect("div should have children");

    assert_eq!(children.len(), 2);
    assert_eq!(children[0].html_type, "span");
    assert_eq!(children[1].html_type, "span");

    let first_text = children[0].props.children.as_ref().unwrap()[0]
        .props
        .text
        .as_deref();
    let second_text = children[1].props.children.as_ref().unwrap()[0]
        .props
        .text
        .as_deref();

    assert_eq!(first_text, Some("first"));
    assert_eq!(second_text, Some("second"));
}
