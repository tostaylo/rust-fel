use crate::a_component::AComponent;
use crate::list::list;
use crate::rustact;
use crate::rustact::Render;
use crate::use_component::use_component;

pub fn app() -> rustact::Element {
    let html = rustact::html(
        "<h5><span><span><p></p></span></span><h1><h2></h2><h3><h4></h4></h3></h1></h5>".to_owned(),
    );

    let a_component = AComponent::new(Some("a-component".to_owned()));

    let app_title = rustact::create_element(
        "TEXT_ELEMENT".to_owned(),
        rustact::Props {
            text: Some("rustact".to_owned()),
            ..Default::default()
        },
    );

    let app = rustact::create_element(
        "div".to_owned(),
        rustact::Props {
            class_name: Some("app".to_owned()),
            children: Some(vec![
                a_component.render(),
                app_title,
                list(),
                html,
                use_component().render(),
            ]),
            ..Default::default()
        },
    );

    app
}
