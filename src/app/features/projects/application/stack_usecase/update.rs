use crate::app::features::projects::domain::{
    entity::NewStack, error::ProjectError, repository::ProjectRepository,
};
use crate::app::features::projects::interface::dto::StackResponseDto;
use crate::app::features::projects::interface::dto::UpdateStackRequestDto;
use std::sync::Arc;
use validator::ValidationError;
use validator::ValidationErrors;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn ProjectRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn ProjectRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub fn execute(
        &self,
        id: i32,
        dto: UpdateStackRequestDto,
    ) -> Result<StackResponseDto, ProjectError> {
        let _existing = self
            .repository
            .get_stack_by_id(id)
            .map_err(|e| ProjectError::System(e.to_string()))?
            .ok_or_else(|| ProjectError::NotFound("Stack not found".to_string()))?;

        // Check uniqueness
        if let Some(existing_stack) = self
            .repository
            .get_stack_by_name(&dto.nama_stack)
            .map_err(|e| ProjectError::System(e.to_string()))?
        {
            if existing_stack.id != id {
                let mut errors = ValidationErrors::new();
                let mut err = ValidationError::new("nama_stack");
                err.message = Some("Stack name already exists".into());
                errors.add("nama_stack", err);
                return Err(ProjectError::Validation(errors));
            }
        }

        let stack = NewStack {
            nama_stack: dto.nama_stack,
        };

        let updated_stack = self
            .repository
            .update_stack(id, stack)
            .map_err(|e| ProjectError::System(e.to_string()))?;

        Ok(StackResponseDto {
            id: updated_stack.id,
            nama_stack: updated_stack.nama_stack,
        })
    }
}
