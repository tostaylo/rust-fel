use crate::log;
use crate::reducer::State;
use crate::rustact;

pub fn app<'a>() -> rustact::Element<'a> {
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

    // let (state, mut dispatch) = rustact::use_reducer(&initial_state);
    // dispatch("reverse");

    let initial_state = 5;
    let (state, mut set_state) = rustact::use_state(initial_state);

    let list_item_2 = rustact::create_element(
        "li",
        rustact::Props {
            children: Some(vec![bye_text]),
            // on_click: Some(&handle_dispatch),
            ..Default::default()
        },
    );
    set_state(5);
    set_state(5);

    let list_items = match *state.borrow() {
        5 => vec![list_item_1, list_item_2],
        10 => vec![list_item_2, list_item_1],
        _ => vec![],
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
