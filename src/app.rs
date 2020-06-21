use crate::list::list;
use crate::rustact;

pub fn app() -> rustact::Element {
    let html = rustact::html(
        "<h5><span><span><p></p></span></span><h1><h2></h2><h3><h4></h4></h3></h1></h5>".to_owned(),
    );

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
            children: Some(vec![app_title, list(), html]),
            ..Default::default()
        },
    );

    app
}
