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

    pub async fn execute(&self, id: i32) -> Result<TagResponseDto, String> {
        let tag = self
            .repository
            .get_tag_by_id(id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Tag not found".to_string())?;

        Ok(TagResponseDto {
            id: tag.id,
            name: tag.name,
            created_at: tag.created_at.to_string(),
            updated_at: tag.updated_at.to_string(),
        })
    }
}
