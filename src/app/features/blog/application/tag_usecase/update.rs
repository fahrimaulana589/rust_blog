use crate::app::features::blog::domain::entity::NewTag;
use crate::app::features::blog::domain::error::BlogError;
use crate::app::features::blog::domain::repository::BlogRepository;
use crate::app::features::blog::interface::dto::{TagResponseDto, UpdateTagRequestDto};
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

    pub async fn execute(
        &self,
        id: i32,
        dto: UpdateTagRequestDto,
    ) -> Result<TagResponseDto, BlogError> {
        let existing = self
            .repository
            .get_tag_by_id(id)
            .map_err(|e| BlogError::System(e.to_string()))?
            .ok_or_else(|| BlogError::NotFound("Tag not found".to_string()))?;

        let mut validation_errors = match dto.validate() {
            Ok(_) => ValidationErrors::new(),
            Err(e) => e,
        };

        if let Some(name) = &dto.name {
            if let Some(existing_tag) = self
                .repository
                .get_tag_by_name(name.clone())
                .map_err(|e| BlogError::System(e.to_string()))?
            {
                if existing_tag.id != id {
                    validation_errors.add("name", ValidationError::new("Tag name already exists"));
                }
            }
        }

        if !validation_errors.is_empty() {
            return Err(BlogError::Validation(validation_errors));
        }

        let new_tag = NewTag {
            name: dto.name.unwrap_or(existing.name),
        };
        let updated_tag = self
            .repository
            .update_tag(id, new_tag)
            .map_err(|e| BlogError::System(e.to_string()))?;

        Ok(TagResponseDto {
            id: updated_tag.id,
            name: updated_tag.name,
            created_at: updated_tag.created_at.to_string(),
            updated_at: updated_tag.updated_at.to_string(),
        })
    }
}
