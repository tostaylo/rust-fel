use crate::log;
use crate::rustact;
pub fn app<'a>(props: bool) -> rustact::Element<'a> {
    let hi_text = rustact::create_element(
        "TEXT_ELEMENT",
        rustact::Props {
            text: Some("Hi from rustact"),
            ..Default::default()
        },
    );

    let bye_text = rustact::create_element(
        "TEXT_ELEMENT",
        rustact::Props {
            text: Some("Bye from rustact"),
            ..Default::default()
        },
    );
    fn logs_on_click() {
        log("I'm list item one");
    }

    let list_item_1 = rustact::create_element(
        "li",
        rustact::Props {
            children: Some(vec![hi_text]),
            on_click: Some(&logs_on_click),
            ..Default::default()
        },
    );

    let list_item_2 = rustact::create_element(
        "li",
        rustact::Props {
            children: Some(vec![bye_text]),
            on_click: Some(&rustact::set_state),
            ..Default::default()
        },
    );

    let list_items = match props {
        true => vec![list_item_1, list_item_2],
        false => vec![list_item_2, list_item_1],
    };

    let list = rustact::create_element(
        "ul",
        rustact::Props {
            children: Some(list_items),
            ..Default::default()
        },
    );

    let app_title = rustact::create_element(
        "TEXT_ELEMENT",
        rustact::Props {
            text: Some("rustact"),
            ..Default::default()
        },
    );

    let app = rustact::create_element(
        "div",
        rustact::Props {
            class_name: Some("app"),
            children: Some(vec![app_title, list]),
            ..Default::default()
        },
    );

    app
}
