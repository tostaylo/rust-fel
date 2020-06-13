use crate::state::State;

pub fn reducer(state: &State, action: &str) -> State {
    match action {
        "reverse" => State { order: false },
        "initial" => State { order: true },
        _ => State { ..state.clone() },
    }
}
