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
    pub excerpt: Option<String>,
    pub thumbnail: Option<String>,
    pub status: Option<String>, // "DRAFT" or "PUBLISHED"
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateBlogRequestDto {
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,
    #[validate(length(min = 1, message = "Content is required"))]
    pub content: String,
    pub category_id: i32,
    pub tag_ids: Option<Vec<i32>>,
    pub excerpt: Option<String>,
    pub thumbnail: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct BlogResponseDto {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub thumbnail: Option<String>,
    pub status: String,
    pub view_count: i32,
    pub category: CategoryResponseDto,
    pub tags: Vec<TagResponseDto>,
    pub created_at: String,
    pub updated_at: String,
    pub published_at: Option<String>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct PaginationRequestDto {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct MetaDto {
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
    pub total_items: i64,
}

#[derive(Deserialize, Serialize)]
pub struct PaginatedResponseDto<T> {
    pub items: Vec<T>,
    pub meta: MetaDto,
}
