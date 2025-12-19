use actix_web::web;
use crate::app;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(app::features::home::handle::index)
    .service(app::features::home::handle::count)
    .default_service(web::get().to(|| async { "Not Found" }));
}