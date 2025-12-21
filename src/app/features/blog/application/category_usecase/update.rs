use crate::app::features::blog::domain::entity::NewCategory;
use crate::app::features::blog::domain::repository::BlogRepository;
use crate::app::features::blog::interface::dto::UpdateCategoryRequestDto;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn BlogRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn BlogRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: i32, dto: UpdateCategoryRequestDto) -> Result<(), String> {
        let existing = self
            .repository
            .get_category_by_id(id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Category not found".to_string())?;

        let new_category = NewCategory {
            name: dto.name.unwrap_or(existing.name),
        };
        self.repository
            .update_category(id, new_category)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
