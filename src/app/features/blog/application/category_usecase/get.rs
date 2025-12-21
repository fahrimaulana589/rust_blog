use crate::app::features::blog::domain::repository::BlogRepository;
use crate::app::features::blog::interface::dto::CategoryResponseDto;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn BlogRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn BlogRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: i32) -> Result<CategoryResponseDto, String> {
        let category = self
            .repository
            .get_category_by_id(id)
            .map_err(|e| e.to_string())?
            .ok_or("Category not found".to_string())?;

        Ok(CategoryResponseDto {
            id: category.id,
            name: category.name,
            created_at: category.created_at.to_string(),
            updated_at: category.updated_at.to_string(),
        })
    }
}
