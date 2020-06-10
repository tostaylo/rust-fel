use crate::log;
use crate::reducer::State;
use crate::rustact;

pub fn list(rustact_struct: rustact::Rustact<State>) -> rustact::Element {
    fn logs_on_click() {
        log("I'm list item one");
    }

    fn reducer(state: &State, action: &str) -> State {
        log(&format!("{:?} {:?} inside reduce", state, action));
        match action {
            "reverse" => State { order: false },
            "initial" => State { order: true },
            _ => State { ..state.clone() },
        }
    }
    log(&format!("{:?} rustact struct from list", rustact_struct));
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

    let list_item_2 = rustact::create_element(
        "li".to_owned(),
        rustact::Props {
            children: Some(vec![bye_text]),
            on_click: Some(Box::new(move || handler(rustact_struct, Box::new(reducer)))),
            ..Default::default()
        },
    );

    let list_items = match rustact_struct.store.order {
        true => vec![list_item_1, list_item_2],
        false => vec![list_item_2, list_item_1],
    };

    let listt = rustact::create_element(
        "ul".to_owned(),
        rustact::Props {
            children: Some(list_items),
            ..Default::default()
        },
    );
    fn handler(mut rustact_struct: rustact::Rustact<State>, reducer: rustact::Reducer<State>) {
        rustact_struct.reduce(Box::new(reducer), "initial");
    }
    listt
}
