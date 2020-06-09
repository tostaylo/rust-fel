use crate::list::list;
use crate::log;
use crate::reducer::State;
use crate::rustact;
use std::{cell::RefCell, rc::Rc};

pub fn app(props: State) -> rustact::Element {
    let mut use_state = rustact::rustact();
    let (state, mut set_state) = use_state(5);
    set_state(5);
    log(&format!("{:?}", state));
    set_state(5);
    log(&format!("{:?}", state));

    // if *state.borrow() >= 15 {
    //     rustact::re_render(props);
    // }

    let app_title = rustact::create_element(
        "TEXT_ELEMENT".to_owned(),
        rustact::Props {
            text: Some("rustact".to_owned()),
            ..Default::default()
        },
    );

    let app = rustact::create_element(
        "div".to_owned(),
        rustact::Props {
            class_name: Some("app".to_owned()),
            children: Some(vec![app_title, list(props)]),
            ..Default::default()
        },
    );

    log(&format!("{:?}", props));
    app
}
