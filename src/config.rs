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
    pub _smtp_username: String,
    pub _smtp_password: String,
    pub smtp_host: String,
    pub smtp_port: i32,
    pub smtp_from: String,
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
            _smtp_username: env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set"),
            _smtp_password: env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set"),
            smtp_host: env::var("SMTP_HOST").expect("SMTP_HOST must be set"),
            smtp_port: env::var("SMTP_PORT").expect("SMTP_PORT must be set").parse().unwrap(),
            smtp_from: env::var("SMTP_FROM").expect("SMTP_FROM must be set"),
        }
    }
}
