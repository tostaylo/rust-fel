use super::html;

#[test]
fn html_preserves_attributes_and_nested_text() {
    let view = html(
        "<main |id=app class=page|><button |type=button data-cy=increment|>Add</button></main>"
            .to_owned(),
    );

    assert_eq!(view.html_type, "main");
    assert_eq!(view.props.id.as_deref(), Some("app"));
    assert_eq!(view.props.class_name.as_deref(), Some("page"));

    let children = view.props.children.expect("main should have children");
    assert_eq!(children.len(), 1);
    assert_eq!(children[0].html_type, "button");
    assert_eq!(children[0].props.type_attr.as_deref(), Some("button"));
    assert_eq!(children[0].props.data_cy.as_deref(), Some("increment"));

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
