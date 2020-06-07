use crate::list::list;
use crate::log;
use crate::reducer::State;
use crate::rustact;

pub fn app(props: State) -> rustact::Element {
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
            children: Some(vec![app_title, list(props)]),
            ..Default::default()
        },
    );

    log(&format!("{:?}", props));
    app
}
