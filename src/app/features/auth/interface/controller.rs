use crate::app::features::auth::interface::dto::{
    ForgotPasswordRequestDto, LoginRequestDto, ResetPasswordRequestDto,
};
use actix_web::{HttpResponse, Responder, post, web};

use validator::Validate;

use crate::utils::di::Container;
use crate::utils::error_response::{map_string_error, map_validation_error};
use crate::utils::success_response::{map_success_response, map_success_with_data};

#[post("/login")]
async fn login(
    container: web::Data<Container>,
    payload: web::Json<LoginRequestDto>,
) -> impl Responder {
    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(map_validation_error(e));
    }
    match container
        .login_usecase
        .execute(payload.username.clone(), payload.password.clone())
    {
        Ok(data) => {
            HttpResponse::Ok().json(map_success_with_data("Login successful".to_string(), data))
        }
        Err(e) => HttpResponse::InternalServerError().json(map_string_error(e)),
    }
}

#[post("/forgot-password")]
async fn forgot_password(
    container: web::Data<Container>,
    payload: web::Json<ForgotPasswordRequestDto>,
) -> impl Responder {
    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(map_validation_error(e));
    }
    match container
        .forgot_password_usecase
        .execute(payload.email.clone())
    {
        Ok(message) => HttpResponse::Ok().json(map_success_response(message)),
        Err(e) => HttpResponse::InternalServerError().json(map_string_error(e)),
    }
}

#[post("/reset-password")]
async fn reset_password(
    container: web::Data<Container>,
    payload: web::Json<ResetPasswordRequestDto>,
) -> impl Responder {
    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(map_validation_error(e));
    }
    match container
        .reset_password_usecase
        .execute(payload.token.clone(), payload.new_password.clone())
    {
        Ok(message) => HttpResponse::Ok().json(map_success_response(message)),
        Err(e) => HttpResponse::InternalServerError().json(map_string_error(e)),
    }
}
