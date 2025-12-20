use crate::app::features::auth::domain::entity::User;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate, Serialize)]
pub struct LoginRequestDto {
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponseDto {
    pub username: String,
    pub email: String,
    pub token: String,
}

impl UserResponseDto {
    pub fn from(user: User, token: String) -> Self {
        Self {
            username: user.username,
            email: user.email,
            token: token,
        }
    }
}
