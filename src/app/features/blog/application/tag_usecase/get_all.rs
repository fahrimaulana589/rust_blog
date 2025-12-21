use crate::app::features::blog::domain::repository::BlogRepository;
use crate::app::features::blog::interface::dto::TagResponseDto;
use std::sync::Arc;

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
        query: crate::app::features::blog::interface::dto::PaginationRequestDto,
    ) -> Result<
        crate::app::features::blog::interface::dto::PaginatedResponseDto<TagResponseDto>,
        String,
    > {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(10);
        let offset = (page - 1) * per_page;

        let (tags, total_items) = self
            .repository
            .get_all_tag(per_page, offset)
            .map_err(|e| e.to_string())?;

        let tag_dtos = tags
            .into_iter()
            .map(|tag| TagResponseDto {
                id: tag.id,
                name: tag.name,
                created_at: tag.created_at.to_string(),
                updated_at: tag.updated_at.to_string(),
            })
            .collect();

        let total_pages = (total_items as f64 / per_page as f64).ceil() as i64;

        Ok(
            crate::app::features::blog::interface::dto::PaginatedResponseDto {
                items: tag_dtos,
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
