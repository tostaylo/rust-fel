pub trait Reducer {
    fn reduce(&self, action: &str) -> State;
}
#[derive(Debug, Default, Clone, Copy)]
pub struct State {
    pub order: bool,
    // pub dispatch:
}

impl Reducer for State {
    fn reduce(&self, action: &str) -> State {
        match action {
            "reverse" => State { order: false },
            "initial" => State { order: true },
            _ => State { ..self.clone() },
        }
    }
}
