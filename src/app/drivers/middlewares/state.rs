use crate::utils::di::Container;

pub struct State {
    pub container : Container
}

impl State {
    pub fn new() -> Self {
        let container = Container::new();
        Self { container }
    }
}