use super::R;
use crate::log;
use crate::rustact;

pub fn list_item_1() -> rustact::Element {
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

    let list_item_1 = rustact::create_element(
        "li".to_owned(),
        rustact::Props {
            children: Some(vec![hi_text]),
            on_click: Some(Box::new(logs_on_click)),
            ..Default::default()
        },
    );
    list_item_1
}
