use crate::app;
use crate::utils::error_response::map_string_error;
use actix_web::{web, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.app_data(
        web::JsonConfig::default().error_handler(crate::utils::error_response::json_error_handler),
    )
    .service(app::features::home::interface::controllers::index)
    .service(
        web::scope("/app")
            .wrap(app::drivers::middlewares::auth::Auth)
            .service(app::features::home::interface::controllers::count),
    )
    .service(app::features::auth::interface::controller::login)
    .default_service(web::get().to(|| async { 
        let error_response = map_string_error("Not Found".to_string());
        HttpResponse::NotFound().json(error_response) 
    }));
}
