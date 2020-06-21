use crate::list::list;
use crate::rustact;

pub fn app() -> rustact::Element {
    let html = "<h5><span><span><p></p></span></span><h1></h1></h5>";

    let arena_tree = rustact::parse_with_stack(html.to_owned());
    let el = arena_tree.create_elements_from_tree();

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
            children: Some(vec![app_title, list(), el]),
            ..Default::default()
        },
    );

    app
}
