mod config;
mod app;
mod utils;

use actix_web::{App, HttpServer};
use app::drivers::middlewares::state::State;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = State::new();
    let url = state.container.config.url.clone();
    println!("server running on {}", url);

    HttpServer::new(move || {
        App::new()
            .app_data(state.container.clone())
            .configure(app::drivers::routes::routes)
    })
    .bind(url)?
    .run()
    .await
}