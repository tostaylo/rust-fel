use crate::list_item::list_item;
use crate::log;
use crate::rustact;

pub fn list_item_3() -> rustact::Element {
    let some_text = rustact::create_element(
        "TEXT_ELEMENT".to_owned(),
        rustact::Props {
            text: Some("Something else from rustact".to_owned()),
            ..Default::default()
        },
    );

    let props = rustact::Props {
        class_name: Some("li".to_owned()),
        children: Some(vec![some_text]),
        ..Default::default()
    };

    list_item(props)
}
