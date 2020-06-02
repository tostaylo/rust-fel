pub trait Reducer {
    fn reduce(&self, action: &str) -> Self;
}
#[derive(Debug, Default, Clone)]
pub struct State {
    pub order: bool,
}

impl Reducer for State {
    fn reduce(&self, action: &str) -> Self {
        match action {
            "reverse" => State {
                order: false,
                ..self.clone()
            },
            "initial" => State {
                order: true,
                ..self.clone()
            },
            _ => State { ..self.clone() },
        }
    }
}
