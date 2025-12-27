use crate::app::features::projects::domain::{
    entity::NewStack, error::ProjectError, repository::ProjectRepository,
};
use crate::app::features::projects::interface::dto::{CreateStackRequestDto, StackResponseDto};
use std::sync::Arc;
use validator::{ValidationError, ValidationErrors};

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn ProjectRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn ProjectRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, dto: CreateStackRequestDto) -> Result<StackResponseDto, ProjectError> {
        // Check uniqueness
        if self
            .repository
            .get_stack_by_name(&dto.nama_stack)
            .map_err(|e| ProjectError::System(e.to_string()))?
            .is_some()
        {
            let mut errors = ValidationErrors::new();
            let mut err = ValidationError::new("nama_stack");
            err.message = Some("Stack name already exists".into());
            errors.add("nama_stack", err);
            return Err(ProjectError::Validation(errors));
        }

        let new_stack = NewStack {
            nama_stack: dto.nama_stack,
        };

        let stack = self
            .repository
            .create_stack(new_stack)
            .map_err(|e| ProjectError::System(e.to_string()))?;

        Ok(StackResponseDto {
            id: stack.id,
            nama_stack: stack.nama_stack,
        })
    }
}
