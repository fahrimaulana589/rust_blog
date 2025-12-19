use std::env;
use dotenv::dotenv;

#[derive(Clone)]
pub struct Config {
    pub url: String,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Config {
        dotenv().ok();
        Config {
            url: env::var("URL").unwrap(),
            database_url: env::var("DATABASE_URL").unwrap(),
        }
    }
}
