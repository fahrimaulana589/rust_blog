use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct CreateStackRequestDto {
    #[validate(length(min = 1, message = "Nama stack is required"))]
    pub nama_stack: String,
}

#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct UpdateStackRequestDto {
    #[validate(length(min = 1, message = "Nama stack is required"))]
    pub nama_stack: String,
}

#[derive(Deserialize, Serialize, Validate, ToSchema, IntoParams)]
pub struct PaginationRequestDto {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct StackResponseDto {
    pub id: i32,
    pub nama_stack: String,
}

#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct CreateProjectRequestDto {
    #[validate(length(min = 1, message = "Nama projek is required"))]
    pub nama_projek: String,
    #[validate(length(min = 1, message = "Deskripsi is required"))]
    pub deskripsi: String,
    #[validate(custom(function = "validate_status"))]
    pub status: String, // draft, ongoing, completed
    #[validate(range(min = 0, max = 100, message = "Progress must be between 0 and 100"))]
    pub progress: i32,
    pub link_demo: Option<String>,
    pub repository: Option<String>,
    #[validate(length(min = 1, message = "Tanggal mulai is required"))]
    pub tanggal_mulai: String, // YYYY-MM-DD
    pub tanggal_selesai: Option<String>, // YYYY-MM-DD
    pub stack_ids: Option<Vec<i32>>,
}

#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct UpdateProjectRequestDto {
    #[validate(length(min = 1, message = "Nama projek is required"))]
    pub nama_projek: String,
    #[validate(length(min = 1, message = "Deskripsi is required"))]
    pub deskripsi: String,
    #[validate(custom(function = "validate_status"))]
    pub status: String,
    #[validate(range(min = 0, max = 100, message = "Progress must be between 0 and 100"))]
    pub progress: Option<i32>,
    pub link_demo: Option<String>,
    pub repository: Option<String>,
    #[validate(length(min = 1, message = "Tanggal mulai is required"))]
    pub tanggal_mulai: String,
    pub tanggal_selesai: Option<String>,
    pub stack_ids: Option<Vec<i32>>,
}

fn validate_status(status: &str) -> Result<(), validator::ValidationError> {
    match status {
        "DRAFT" | "ONGOING" | "COMPLETED" => Ok(()),
        _ => Err(validator::ValidationError::new("Invalid status")),
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct MetaDto {
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
    pub total_items: i64,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct PaginatedResponseDto<T> {
    pub items: Vec<T>,
    pub meta: MetaDto,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ProjectResponseDto {
    pub id: i32,
    pub nama_projek: String,
    pub deskripsi: String,
    pub status: String,
    pub progress: i32,
    pub link_demo: Option<String>,
    pub repository: Option<String>,
    pub tanggal_mulai: String,
    pub tanggal_selesai: Option<String>,
    pub stacks: Vec<StackResponseDto>,
    pub created_at: String,
    pub updated_at: String,
}
