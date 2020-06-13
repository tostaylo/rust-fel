#[derive(Debug, Default, Clone, Copy)]
pub struct State {
    pub order: bool,
}

impl State {
    pub fn new(order: bool) -> Self {
        Self { order }
    }
}
