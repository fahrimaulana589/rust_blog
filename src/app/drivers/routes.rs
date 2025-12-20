use crate::app;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.app_data(
        web::JsonConfig::default().error_handler(crate::utils::error_response::json_error_handler),
    )
    .service(app::features::home::interface::controllers::index)
    .service(app::features::home::interface::controllers::count)
    .service(app::features::auth::interface::controller::login)
    .default_service(web::get().to(|| async { "Not Found" }));
}
