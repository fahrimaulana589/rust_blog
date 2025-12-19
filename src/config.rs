use std::env;

pub struct Config {
    pub url: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            url: env::var("URL").unwrap(),
        }
    }
}
