mod app;
mod config;
mod schema;
mod utils;

#[cfg(test)]
mod test;

use actix_cors::Cors;
use actix_web::{App, HttpServer, http, web};
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
        let mut cors = Cors::default();

        for origin in &state.container.config.allowed_origins {
            cors = cors.allowed_origin(origin);
        }

        let cors = cors
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
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
