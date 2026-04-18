use super::Props;

#[test]
fn default_props_start_empty() {
    let props = Props::default();

    assert!(props.children.is_none());
    assert!(props.text.is_none());
    assert!(props.on_click.is_none());
    assert!(props.mouse.is_none());
    assert!(props.attributes.is_none());
}

#[test]
fn debug_output_mentions_core_fields() {
    let mut props = Props {
        text: Some("hello".to_owned()),
        ..Default::default()
    };
    props.set_attribute("aria-label", "Greeting");
    props.set_attribute("class", "banner");
    props.set_attribute("id", "hero");

    let debug = format!("{props:?}");

    assert!(debug.contains("hello"));
    assert!(debug.contains("aria-label"));
    assert!(debug.contains("banner"));
    assert!(debug.contains("hero"));
}
