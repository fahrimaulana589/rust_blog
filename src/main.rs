mod app;
mod config;
mod schema;
mod utils;

#[cfg(test)]
mod test;

use actix_web::{App, HttpServer, web};
use app::drivers::middlewares::state::State;

use app::drivers::openapi::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = State::new();
    let url = state.container.config.url.clone();
    println!("server running on {}", url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.container.clone()))
            .configure(app::drivers::routes::routes)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(url)?
    .run()
    .await
}
