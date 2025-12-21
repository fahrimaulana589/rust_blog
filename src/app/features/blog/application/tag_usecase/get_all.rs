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

    pub async fn execute(&self) -> Result<Vec<TagResponseDto>, String> {
        let tags = self.repository.get_all_tag().map_err(|e| e.to_string())?;

        let tag_dtos = tags
            .into_iter()
            .map(|tag| TagResponseDto {
                id: tag.id,
                name: tag.name,
                created_at: tag.created_at.to_string(),
                updated_at: tag.updated_at.to_string(),
            })
            .collect();

        Ok(tag_dtos)
    }
}
