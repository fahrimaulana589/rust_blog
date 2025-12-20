use crate::app::features::auth::interface::dto::{LoginRequestDto};
use actix_web::{HttpResponse, post, web};

use validator::Validate;

use crate::utils::di::Container;
use crate::utils::error_response::{map_string_error, map_validation_error};

#[post("/login")]
async fn login(container: web::Data<Container>, body: web::Json<LoginRequestDto>) -> HttpResponse {
    match body.validate() {
        Ok(_) => {
            let result = container
                .login_usecase
                .execute(body.username.clone(), body.password.clone());
            match result {
                Ok(result) => {
                    HttpResponse::Ok().json(result)
                }
                Err(e) => {
                    let error_response = map_string_error(e);
                    HttpResponse::BadRequest().json(error_response)
                }
            }
        }
        Err(e) => {
            let error_response = map_validation_error(e);
            HttpResponse::BadRequest().json(error_response)
        }
    }
}
