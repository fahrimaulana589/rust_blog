use crate::app::features::auth::interface::dto::{
    ForgotPasswordRequestDto, LoginRequestDto, ResetPasswordRequestDto,
};
use actix_web::{HttpResponse, post, web};

use validator::Validate;

use crate::utils::di::Container;
use crate::utils::error_response::{map_string_error, map_validation_error};
use crate::utils::success_response::{map_success_response, map_success_with_data};

#[post("/login")]
async fn login(container: web::Data<Container>, body: web::Json<LoginRequestDto>) -> HttpResponse {
    match body.validate() {
        Ok(_) => {
            let result = container
                .login_usecase
                .execute(body.username.clone(), body.password.clone());
            match result {
                Ok(data) => {
                    let response = map_success_with_data("Login successful".to_string(), data);
                    HttpResponse::Ok().json(response)
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

#[post("/forgot-password")]
async fn forgot_password(
    container: web::Data<Container>,
    body: web::Json<ForgotPasswordRequestDto>,
) -> HttpResponse {
    match body.0.validate() {
        Ok(_) => {
            let result = container
                .forgot_password_usecase
                .execute(body.email.clone());
            match result {
                Ok(message) => {
                    let response = map_success_response(message);
                    HttpResponse::Ok().json(response)
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

#[post("/reset-password")]
async fn reset_password(
    container: web::Data<Container>,
    body: web::Json<ResetPasswordRequestDto>,
) -> HttpResponse {
    match body.0.validate() {
        Ok(_) => {
            let result = container
                .reset_password_usecase
                .execute(body.token.clone(), body.new_password.clone());
            match result {
                Ok(message) => {
                    let response = map_success_response(message);
                    HttpResponse::Ok().json(response)
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
