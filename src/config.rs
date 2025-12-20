use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub url: String,
    pub database_url: String,
    pub jwt_secret: String,
    pub default_username: String,
    pub default_email: String,
    pub default_password: String,
}

impl Config {
    pub fn new() -> Config {
        dotenv().ok();
        Config {
            url: env::var("URL").unwrap(),
            database_url: env::var("DATABASE_URL").unwrap(),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            default_username: env::var("DEFAULT_USERNAME").expect("DEFAULT_USERNAME must be set"),
            default_email: env::var("DEFAULT_EMAIL").expect("DEFAULT_EMAIL must be set"),
            default_password: env::var("DEFAULT_PASSWORD").expect("DEFAULT_PASSWORD must be set"),
        }
    }
}
