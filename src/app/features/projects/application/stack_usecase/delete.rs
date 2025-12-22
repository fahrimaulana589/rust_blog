use crate::app::features::projects::domain::repository::ProjectRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn ProjectRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn ProjectRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, id: i32) -> Result<(), String> {
        // Check existence
        let _existing = self
            .repository
            .get_stack_by_id(id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Stack not found".to_string())?;

        self.repository
            .delete_stack(id)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
