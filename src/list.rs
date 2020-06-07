use crate::log;
use crate::reducer::State;
use crate::rustact;

pub fn list(props: State) -> rustact::Element {
    fn logs_on_click() {
        log("I'm list item one");
    }

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

    let list_item_1 = rustact::create_element(
        "li".to_owned(),
        rustact::Props {
            children: Some(vec![hi_text]),
            on_click: Some(Box::new(logs_on_click)),
            ..Default::default()
        },
    );

    fn handler(val: State) {
        //This should be dispatch
        let val = State { order: !val.order };
        rustact::re_render(val);
    }
    let list_item_2 = rustact::create_element(
        "li".to_owned(),
        rustact::Props {
            children: Some(vec![bye_text]),
            on_click: Some(Box::new(move || handler(props))),
            ..Default::default()
        },
    );

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

    list
}
