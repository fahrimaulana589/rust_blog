use crate::app::features::auth::{
    domain::entity::User,
    interface::dto::{LoginRequestDto, UserResponseDto},
};
use actix_web::{HttpResponse, post, web};

use validator::Validate;

use crate::utils::di::Container;
use crate::utils::error_response::{map_string_error, map_validation_error};

#[post("/login")]
async fn login(container: web::Data<Container>, body: web::Json<LoginRequestDto>) -> HttpResponse {
    match body.validate() {
        Ok(_) => {
            let user = container
                .login_usecase
                .execute(body.username.clone(), body.password.clone());
            match user {
                Ok(user) => HttpResponse::Ok().json(UserResponseDto::from(user)),
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
