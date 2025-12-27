use crate::app::features::blog::domain::repository::BlogRepository;
use crate::app::features::blog::interface::dto::CategoryResponseDto;
use std::sync::Arc;
use crate::app::features::blog::interface::dto::PaginationRequestDto;
use crate::app::features::blog::interface::dto::PaginatedResponseDto;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn BlogRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn BlogRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        query: PaginationRequestDto,
    ) -> Result<
        PaginatedResponseDto<CategoryResponseDto>,
        String,
    > {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(10);
        let offset = (page - 1) * per_page;

        let (categories, total_items) = self
            .repository
            .get_all_category(per_page, offset)
            .map_err(|e| e.to_string())?;

        let category_dtos = categories
            .into_iter()
            .map(|c| CategoryResponseDto {
                id: c.id,
                name: c.name,
                created_at: c.created_at.to_string(),
                updated_at: c.updated_at.to_string(),
            })
            .collect();

        let total_pages = (total_items as f64 / per_page as f64).ceil() as i64;

        Ok(
            crate::app::features::blog::interface::dto::PaginatedResponseDto {
                items: category_dtos,
                meta: crate::app::features::blog::interface::dto::MetaDto {
                    page,
                    per_page,
                    total_pages,
                    total_items,
                },
            },
        )
    }
}
