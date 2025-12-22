use crate::app::features::auth::domain::entity::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, Serialize, ToSchema)]
pub struct LoginRequestDto {
    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Deserialize, Validate, Serialize, ToSchema)]
pub struct ForgotPasswordRequestDto {
    #[validate(length(min = 1, message = "Email is required"))]
    #[validate(email(message = "Email is invalid"))]
    pub email: String,
}

#[derive(Deserialize, Validate, Serialize, ToSchema)]
pub struct ResetPasswordRequestDto {
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,
    #[validate(length(min = 1, message = "New password is required"))]
    pub new_password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginResponseDto {
    pub username: String,
    pub email: String,
    pub token: String,
}

impl LoginResponseDto {
    pub fn from(user: User, token: String) -> Self {
        Self {
            username: user.username,
            email: user.email,
            token: token,
        }
    }
}
