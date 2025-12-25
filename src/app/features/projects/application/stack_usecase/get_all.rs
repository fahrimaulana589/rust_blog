use crate::app::features::projects::domain::repository::ProjectRepository;
use crate::app::features::projects::interface::dto::{
    MetaDto, PaginatedResponseDto, StackResponseDto,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn ProjectRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn ProjectRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub fn execute(
        &self,
        page: i64,
        per_page: i64,
    ) -> Result<PaginatedResponseDto<StackResponseDto>, String> {
        let limit = per_page;
        let offset = (page - 1) * per_page;

        let (stacks, total_count) = self
            .repository
            .get_all_stacks(limit, offset)
            .map_err(|e| e.to_string())?;

        let items: Vec<StackResponseDto> = stacks
            .into_iter()
            .map(|s| StackResponseDto {
                id: s.id,
                nama_stack: s.nama_stack,
            })
            .collect();

        let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;

        Ok(PaginatedResponseDto {
            items,
            meta: MetaDto {
                page,
                per_page,
                total_pages,
                total_items: total_count,
            },
        })
    }
}
