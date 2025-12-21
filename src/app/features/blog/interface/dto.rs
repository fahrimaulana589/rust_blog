use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateCategoryRequestDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateCategoryRequestDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct CategoryResponseDto {
    pub id: i32,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}
