use crate::config::Config;

#[derive(Clone)]
pub struct Container {
    pub config: Config,
}

impl Container {
    pub fn new() -> Self {
        let config = Config::new();
        Self { config }
    }
}