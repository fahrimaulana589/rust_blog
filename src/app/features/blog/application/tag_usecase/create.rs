use crate::app::features::blog::domain::entity::NewTag;
use crate::app::features::blog::domain::repository::BlogRepository;
use crate::app::features::blog::interface::dto::CreateTagRequestDto;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn BlogRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn BlogRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, dto: CreateTagRequestDto) -> Result<(), String> {
        let new_tag = NewTag { name: dto.name };
        self.repository
            .create_tag(new_tag)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
