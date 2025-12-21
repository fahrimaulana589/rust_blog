use crate::app;
use crate::utils::error_response::map_string_error;
use actix_web::{HttpResponse, web};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.app_data(
        web::JsonConfig::default().error_handler(crate::utils::error_response::json_error_handler),
    )
    .service(app::features::home::interface::controllers::index)
    .service(
        web::scope("/app")
            .wrap(app::drivers::middlewares::auth::Auth)
            .service(app::features::home::interface::controllers::count)
            .service(app::features::home::interface::controllers::send_email)
            .service(app::features::blog::interface::controller::create_category)
            .service(app::features::blog::interface::controller::get_categories)
            .service(app::features::blog::interface::controller::get_category)
            .service(app::features::blog::interface::controller::update_category)
            .service(app::features::blog::interface::controller::delete_category)
            .service(app::features::blog::interface::controller::create_tag)
            .service(app::features::blog::interface::controller::get_tags)
            .service(app::features::blog::interface::controller::get_tag)
            .service(app::features::blog::interface::controller::update_tag)
            .service(app::features::blog::interface::controller::delete_tag)
            .service(app::features::blog::interface::controller::create_blog)
            .service(app::features::blog::interface::controller::get_blogs)
            .service(app::features::blog::interface::controller::get_blog)
            .service(app::features::blog::interface::controller::update_blog)
            .service(app::features::blog::interface::controller::delete_blog),
    )
    .service(app::features::auth::interface::controller::login)
    .service(app::features::auth::interface::controller::forgot_password)
    .service(app::features::auth::interface::controller::reset_password)
    .default_service(web::get().to(|| async {
        let error_response = map_string_error("Not Found".to_string());
        HttpResponse::NotFound().json(error_response)
    }));
}
