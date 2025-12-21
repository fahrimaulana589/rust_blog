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

    pub async fn execute(&self) -> Result<Vec<CategoryResponseDto>, String> {
        let categories = self
            .repository
            .get_all_category()
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

        Ok(category_dtos)
    }
}
