#[derive(Debug, Clone)]
pub enum Action {
    Increment,
    Decrement,
}

impl Default for Action {
    fn default() -> Self {
        Action::Increment
    }
}
