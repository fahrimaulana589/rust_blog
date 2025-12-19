mod config;
mod app;
mod utils;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = Config::new();

    println!("server running on {}", config.url);

    HttpServer::new(|| {
        App::new()
            .configure(app::drivers::routes::routes)
    })
    .bind(config.url)?
    .run()
    .await
}