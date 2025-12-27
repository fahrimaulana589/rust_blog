use crate::app::features::blog::domain::entity::NewTag;
use crate::app::features::blog::domain::error::BlogError;
use crate::app::features::blog::domain::repository::BlogRepository;
use crate::app::features::blog::interface::dto::{CreateTagRequestDto, TagResponseDto};
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

    pub async fn execute(&self, dto: CreateTagRequestDto) -> Result<TagResponseDto, BlogError> {
        let mut validation_errors = match dto.validate() {
            Ok(_) => ValidationErrors::new(),
            Err(e) => e,
        };

        if self
            .repository
            .get_tag_by_name(dto.name.clone())
            .map_err(|e| BlogError::System(e.to_string()))?
            .is_some()
        {
            validation_errors.add("name", ValidationError::new("Tag name already exists"));
        }

        if !validation_errors.is_empty() {
            return Err(BlogError::Validation(validation_errors));
        }

        let new_tag = NewTag { name: dto.name };
        let created_tag = self
            .repository
            .create_tag(new_tag)
            .map_err(|e| BlogError::System(e.to_string()))?;

        Ok(TagResponseDto {
            id: created_tag.id,
            name: created_tag.name,
            created_at: created_tag.created_at.to_string(),
            updated_at: created_tag.updated_at.to_string(),
        })
    }
}
