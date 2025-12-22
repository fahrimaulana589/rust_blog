use crate::app::features::projects::domain::repository::ProjectRepository;
use crate::app::features::projects::interface::dto::StackResponseDto;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn ProjectRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn ProjectRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, id: i32) -> Result<StackResponseDto, String> {
        let stack = self
            .repository
            .get_stack_by_id(id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Stack not found".to_string())?;

        Ok(StackResponseDto {
            id: stack.id,
            nama_stack: stack.nama_stack,
        })
    }
}
