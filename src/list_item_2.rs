use super::app;
use super::STORE;
use crate::reducer::reducer;
use crate::rustact;
use crate::state::State;

pub fn list_item_2() -> rustact::Element {
    let store = STORE.lock().unwrap().store();

    let action = match store.order {
        true => "reverse",
        false => "initial",
    };

    let bye_text = rustact::create_element(
        "TEXT_ELEMENT".to_owned(),
        rustact::Props {
            text: Some("Bye from rustact".to_owned()),
            ..Default::default()
        },
    );

    fn handler(rustact_struct: STORE, reducer: rustact::Reducer<State>, action: &str) {
        rustact_struct
            .lock()
            .unwrap()
            .reduce(Box::new(reducer), action);
        rustact::re_render(app(), None);
    }

    let list_item_2 = rustact::create_element(
        "li".to_owned(),
        rustact::Props {
            children: Some(vec![bye_text]),
            on_click: Some(Box::new(move || handler(STORE, Box::new(reducer), action))),
            ..Default::default()
        },
    );

    list_item_2
}
