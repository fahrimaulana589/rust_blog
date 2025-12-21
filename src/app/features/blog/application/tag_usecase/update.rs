use crate::app::features::blog::domain::entity::NewTag;
use crate::app::features::blog::domain::repository::BlogRepository;
use crate::app::features::blog::interface::dto::UpdateTagRequestDto;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn BlogRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn BlogRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: i32, dto: UpdateTagRequestDto) -> Result<(), String> {
        let new_tag = NewTag { name: dto.name };
        self.repository
            .update_tag(id, new_tag)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
