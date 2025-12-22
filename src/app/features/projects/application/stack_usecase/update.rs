use crate::app::features::projects::domain::{entity::NewStack, repository::ProjectRepository};
use crate::app::features::projects::interface::dto::UpdateStackRequestDto;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn ProjectRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn ProjectRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, id: i32, dto: UpdateStackRequestDto) -> Result<(), String> {
        let _existing = self
            .repository
            .get_stack_by_id(id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Stack not found".to_string())?;

        let stack = NewStack {
            nama_stack: dto.nama_stack,
        };

        self.repository
            .update_stack(id, stack)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
