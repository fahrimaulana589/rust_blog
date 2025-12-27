use crate::app::features::projects::interface::dto::ProjectResponseDto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct CreatePortofolioRequestDto {
    #[validate(range(min = 1, message = "Project ID is required"))]
    pub project_id: i32,
    #[validate(length(min = 1, message = "Judul is required"))]
    pub judul: String,
    #[validate(length(min = 1, message = "Deskripsi is required"))]
    pub deskripsi: String,
    pub is_active: bool,
}

#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct UpdatePortofolioRequestDto {
    #[validate(range(min = 1, message = "Project ID is required"))]
    pub project_id: i32,
    #[validate(length(min = 1, message = "Judul is required"))]
    pub judul: String,
    #[validate(length(min = 1, message = "Deskripsi is required"))]
    pub deskripsi: String,
    pub is_active: bool,
}

#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct PaginationRequestDto {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
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
pub struct PortofolioResponseDto {
    pub id: i32,
    pub judul: String,
    pub slug: String,
    pub deskripsi: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub project: ProjectResponseDto,
}
