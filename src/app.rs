use crate::log;
use crate::reducer::State;
use crate::rustact;

pub fn app(props: State) -> rustact::Element {
    let hi_text = rustact::create_element(
        "TEXT_ELEMENT".to_owned(),
        rustact::Props {
            text: Some("Hi from rustact".to_owned()),
            ..Default::default()
        },
    );

    let bye_text = rustact::create_element(
        "TEXT_ELEMENT".to_owned(),
        rustact::Props {
            text: Some("Bye from rustact".to_owned()),
            ..Default::default()
        },
    );
    fn logs_on_click() {
        log("I'm list item one");
    }

    let list_item_1 = rustact::create_element(
        "li".to_owned(),
        rustact::Props {
            children: Some(vec![hi_text]),
            on_click: Some(Box::new(logs_on_click)),
            ..Default::default()
        },
    );

    fn handler(val: State) {
        let val = State { order: !val.order };
        rustact::re_render(val);
        ()
    }
    let list_item_2 = rustact::create_element(
        "li".to_owned(),
        rustact::Props {
            children: Some(vec![bye_text]),
            on_click: Some(Box::new(move || handler(props))),
            ..Default::default()
        },
    );
    log(&format!("{:?}", props));

    let list_items = match props.order {
        true => vec![list_item_1, list_item_2],
        false => vec![list_item_2, list_item_1],
    };

    let list = rustact::create_element(
        "ul".to_owned(),
        rustact::Props {
            children: Some(list_items),
            ..Default::default()
        },
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
            children: Some(vec![app_title, list]),
            ..Default::default()
        },
    );

    app
}
