use crate::app::features::blog::domain::entity::NewCategory;
use crate::app::features::blog::domain::error::BlogError;
use crate::app::features::blog::domain::repository::BlogRepository;
use crate::app::features::blog::interface::dto::CreateCategoryRequestDto;
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

    pub async fn execute(&self, dto: CreateCategoryRequestDto) -> Result<(), BlogError> {
        let mut validation_errors = match dto.validate() {
            Ok(_) => ValidationErrors::new(),
            Err(e) => e,
        };

        if self
            .repository
            .get_category_by_name(dto.name.clone())
            .map_err(|e| BlogError::System(e.to_string()))?
            .is_some()
        {
            validation_errors.add("name", ValidationError::new("Category name already exists"));
        }

        if !validation_errors.is_empty() {
            return Err(BlogError::Validation(validation_errors));
        }

        let new_category = NewCategory { name: dto.name };
        self.repository
            .create_category(new_category)
            .map_err(|e| BlogError::System(e.to_string()))?;
        Ok(())
    }
}
