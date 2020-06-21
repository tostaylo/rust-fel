use super::STORE;
use crate::list_item_1::list_item_1;
use crate::list_item_2::list_item_2;
use crate::list_item_3::list_item_3;
use crate::log;
use crate::rustact;

pub fn list() -> rustact::Element {
    let store = STORE.lock().unwrap().store();
    log(&format!("{:?} static", store));
    let l1 = list_item_1();
    let l2 = list_item_2();
    let l3 = list_item_3();

    let list_items = match store.order {
        true => vec![l1, l2, l3],
        false => vec![l2, l1, l3],
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
