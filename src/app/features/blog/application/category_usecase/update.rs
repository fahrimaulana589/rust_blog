use crate::app::features::blog::domain::entity::NewCategory;
use crate::app::features::blog::domain::error::BlogError;
use crate::app::features::blog::domain::repository::BlogRepository;
use crate::app::features::blog::interface::dto::UpdateCategoryRequestDto;
use std::sync::Arc;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn BlogRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn BlogRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: i32, dto: UpdateCategoryRequestDto) -> Result<(), BlogError> {
        let existing = self
            .repository
            .get_category_by_id(id)
            .map_err(|e| BlogError::System(e.to_string()))?
            .ok_or_else(|| BlogError::NotFound("Category not found".to_string()))?;

        let mut validation_errors = match dto.validate() {
            Ok(_) => ValidationErrors::new(),
            Err(e) => e,
        };

        if let Some(name) = &dto.name {
            if let Some(existing_cat) = self
                .repository
                .get_category_by_name(name.clone())
                .map_err(|e| BlogError::System(e.to_string()))?
            {
                if existing_cat.id != id {
                    validation_errors
                        .add("name", ValidationError::new("Category name already exists"));
                }
            }
        }

        if !validation_errors.is_empty() {
            return Err(BlogError::Validation(validation_errors));
        }

        let new_category = NewCategory {
            name: dto.name.unwrap_or(existing.name),
        };
        self.repository
            .update_category(id, new_category)
            .map_err(|e| BlogError::System(e.to_string()))?;
        Ok(())
    }
}
