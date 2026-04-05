use crate::ClosureProp;

#[test]
fn rsx_macro_builds_angle_bracket_elements() {
    let click: ClosureProp = Box::new(|| {});

    let view = crate::rsx! {
        <main id={"counter-app"} class="demo-app">
            <button type="button" data-cy="increment" on_click={click}>{"Increment"}</button>
        </main>
    };

    assert_eq!(view.html_type, "main");
    assert_eq!(view.props.attribute("id"), Some("counter-app"));
    assert_eq!(view.props.attribute("class"), Some("demo-app"));

    let child = view
        .props
        .children
        .as_ref()
        .and_then(|children| children.first())
        .expect("main should contain a button child");

    assert_eq!(child.html_type, "button");
    assert_eq!(child.props.attribute("type"), Some("button"));
    assert_eq!(child.props.attribute("data-cy"), Some("increment"));
    assert!(child.props.on_click.is_some());

    let text_child = child
        .props
        .children
        .as_ref()
        .and_then(|children| children.first())
        .expect("button should contain a text child");

    assert_eq!(text_child.html_type, "TEXT_ELEMENT");
    assert_eq!(text_child.props.text.as_deref(), Some("Increment"));
}