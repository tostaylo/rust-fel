use super::app;
use super::R;
use crate::log;
use crate::reducer::State;
use crate::rustact;

pub fn list_item_2() -> rustact::Element {
    fn reducer(state: &State, action: &str) -> State {
        log(&format!("{:?} {:?} inside the reducer", state, action));
        match action {
            "reverse" => State { order: false },
            "initial" => State { order: true },
            _ => State { ..state.clone() },
        }
    }

    let action = match R.lock().unwrap().store.order {
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

    fn handler(rustact_struct: R, reducer: rustact::Reducer<State>, action: &str) {
        rustact_struct
            .lock()
            .unwrap()
            .reduce(Box::new(reducer), action);
        rustact::re_render(app());
    }

    let list_item_2 = rustact::create_element(
        "li".to_owned(),
        rustact::Props {
            children: Some(vec![bye_text]),
            on_click: Some(Box::new(move || handler(R, Box::new(reducer), action))),
            ..Default::default()
        },
    );

    list_item_2
}
