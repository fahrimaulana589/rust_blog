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
            let cookie = actix_web::cookie::Cookie::build("auth_token", data.token.clone())
                .http_only(true)
                .path("/")
                .max_age(actix_web::cookie::time::Duration::hours(1))
                .finish();

            HttpResponse::Ok()
                .cookie(cookie)
                .json(map_success_with_data("Login successful".to_string(), data))
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

#[utoipa::path(
    get,
    path = "/app/islogin",
    tag = "Auth",
    security(("jwt" = [])),
    responses(
        (status = 200, description = "User is logged in", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
#[actix_web::get("/islogin")]
async fn is_login() -> impl Responder {
    HttpResponse::Ok().json(map_success_response("User is logged in".to_string()))
}

#[utoipa::path(
    post,
    path = "/logout",
    tag = "Auth",
    responses(
        (status = 200, description = "Logout successful", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>),
    )
)]
#[actix_web::post("/logout")]
async fn logout() -> impl Responder {
    let cookie = actix_web::cookie::Cookie::build("auth_token", "")
        .http_only(true)
        .path("/")
        .max_age(actix_web::cookie::time::Duration::milliseconds(0)) // 0 duration to expire immediately
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(map_success_response("Logout successful".to_string()))
}
