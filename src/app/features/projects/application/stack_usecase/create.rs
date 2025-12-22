use crate::app::features::projects::domain::entity::NewStack;
use crate::app::features::projects::domain::repository::ProjectRepository;
use crate::app::features::projects::interface::dto::{CreateStackRequestDto, StackResponseDto};
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn ProjectRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn ProjectRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, dto: CreateStackRequestDto) -> Result<StackResponseDto, String> {
        // Check uniqueness
        if self
            .repository
            .get_stack_by_name(&dto.nama_stack)
            .map_err(|e| e.to_string())?
            .is_some()
        {
            return Err("Stack name already exists".to_string());
        }

        let new_stack = NewStack {
            nama_stack: dto.nama_stack,
        };

        let stack = self
            .repository
            .create_stack(new_stack)
            .map_err(|e| e.to_string())?;

        Ok(StackResponseDto {
            id: stack.id,
            nama_stack: stack.nama_stack,
        })
    }
}
