use crate::app::features::auth::interface::dto::{
    ForgotPasswordRequestDto, LoginRequestDto, LoginResponseDto, ResetPasswordRequestDto,
};
use actix_web::{HttpResponse, Responder, post, web};

use validator::Validate;

use crate::utils::di::Container;
use crate::utils::error_response::{ErrorResponse, map_string_error, map_validation_error};
use crate::utils::success_response::{map_success_response, map_success_with_data};

#[utoipa::path(
    path = "/login",
    tag = "Auth",
    request_body = LoginRequestDto,
    responses(
        (status = 200, description = "Login successful", body = crate::utils::success_response::SuccessResponse<LoginResponseDto>),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
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

#[utoipa::path(
    path = "/forgot-password",
    tag = "Auth",
    request_body = ForgotPasswordRequestDto,
    responses(
        (status = 200, description = "Email sent", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
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

#[utoipa::path(
    path = "/reset-password",
    tag = "Auth",
    request_body = ResetPasswordRequestDto,
    responses(
        (status = 200, description = "Password reset successful", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
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
