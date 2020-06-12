use super::R;
use crate::list_item_1::list_item_1;
use crate::list_item_2::list_item_2;
use crate::log;
use crate::rustact;

pub fn list() -> rustact::Element {
    log(&format!("{:?} static", R.lock().unwrap().store));
    let l1 = list_item_1();
    let l2 = list_item_2();
    let list_items = match R.lock().unwrap().store.order {
        true => vec![l1, l2],
        false => vec![l2, l1],
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
