use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct CreateCategoryRequestDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct UpdateCategoryRequestDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CategoryResponseDto {
    pub id: i32,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct CreateTagRequestDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct UpdateTagRequestDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct TagResponseDto {
    pub id: i32,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct CreateBlogRequestDto {
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,
    #[validate(length(min = 1, message = "Content is required"))]
    pub content: String,
    #[validate(range(min = 1, message = "Category ID is required"))]
    pub category_id: i32,
    pub tag_ids: Option<Vec<i32>>,
    #[validate(length(min = 1, message = "Excerpt is required"))]
    pub excerpt: String,
    pub thumbnail: Option<String>,
    #[validate(length(min = 1, message = "Status is required"),custom(function = "validate_status"))]
    pub status: String, // "DRAFT" or "PUBLISHED"
}

#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct UpdateBlogRequestDto {
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,
    #[validate(length(min = 1, message = "Content is required"))]
    pub content: String,
    #[validate(range(min = 1, message = "Category ID is required"))]
    pub category_id: i32,
    pub tag_ids: Option<Vec<i32>>,
    #[validate(length(min = 1, message = "Excerpt is required"))]
    pub excerpt: String,
    pub thumbnail: Option<String>,
    #[validate(length(min = 1, message = "Status is required"),custom(function = "validate_status"))]
    pub status: String,
}

fn validate_status(status: &str) -> Result<(), ValidationError> {
    match status {
        "DRAFT" | "PUBLISHED" | "ARCHIVED" => Ok(()),
        _ => Err(validator::ValidationError::new("Invalid status")),
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
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
