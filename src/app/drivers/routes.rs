use crate::app;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(app::features::home::interface::controllers::index)
        .service(app::features::home::interface::controllers::count)
        .default_service(web::get().to(|| async { "Not Found" }));
}
