use crate::app::features::blog::domain::entity::NewCategory;
use crate::app::features::blog::domain::repository::BlogRepository;
use crate::app::features::blog::interface::dto::CreateCategoryRequestDto;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn BlogRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn BlogRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, dto: CreateCategoryRequestDto) -> Result<(), String> {
        let new_category = NewCategory { name: dto.name };
        self.repository
            .create_category(new_category)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
