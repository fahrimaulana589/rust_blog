mod app;
mod config;
mod schema;
mod utils;

#[cfg(test)]
mod test;

use actix_web::{App, HttpServer, web};
use app::drivers::middlewares::state::State;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = State::new();
    let url = state.container.config.url.clone();
    println!("server running on {}", url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.container.clone()))
            .configure(app::drivers::routes::routes)
    })
    .bind(url)?
    .run()
    .await
}
