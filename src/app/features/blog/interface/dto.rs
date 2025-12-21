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

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateTagRequestDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateTagRequestDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct TagResponseDto {
    pub id: i32,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateBlogRequestDto {
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,
    #[validate(length(min = 1, message = "Content is required"))]
    pub content: String,
    pub category_id: i32,
    pub tag_ids: Option<Vec<i32>>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateBlogRequestDto {
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,
    #[validate(length(min = 1, message = "Content is required"))]
    pub content: String,
    pub category_id: i32,
    pub tag_ids: Option<Vec<i32>>,
}

#[derive(Deserialize, Serialize)]
pub struct BlogResponseDto {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub category: CategoryResponseDto,
    pub tags: Vec<TagResponseDto>,
    pub created_at: String,
    pub updated_at: String,
}
