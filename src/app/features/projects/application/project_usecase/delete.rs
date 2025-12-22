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
        let _ = self
            .repository
            .get_project_by_id(id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Project not found".to_string())?;

        self.repository
            .delete_project(id)
            .map_err(|e| e.to_string())?;

        // Cascade delete of relations handled by DB constraint usually, but we can double check or let DB handle it.
        // Our schema has ON DELETE CASCADE, so no manual deletion of project_stack needed.
        Ok(())
    }
}
