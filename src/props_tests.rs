use super::Props;

#[test]
fn default_props_start_empty() {
    let props = Props::default();

    assert!(props.children.is_none());
    assert!(props.text.is_none());
    assert!(props.on_click.is_none());
    assert!(props.mouse.is_none());
    assert!(props.class_name.is_none());
    assert!(props.id.is_none());
    assert!(props.href.is_none());
    assert!(props.src.is_none());
    assert!(props.type_attr.is_none());
    assert!(props.role.is_none());
    assert!(props.data_cy.is_none());
}

#[test]
fn debug_output_mentions_core_fields() {
    let props = Props {
        text: Some("hello".to_owned()),
        class_name: Some("banner".to_owned()),
        id: Some("hero".to_owned()),
        ..Default::default()
    };

    let debug = format!("{props:?}");

    assert!(debug.contains("hello"));
    assert!(debug.contains("banner"));
    assert!(debug.contains("hero"));
}
