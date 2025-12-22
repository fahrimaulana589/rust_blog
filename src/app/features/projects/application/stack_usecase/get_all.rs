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

    pub fn execute(&self) -> Result<Vec<StackResponseDto>, String> {
        let stacks = self
            .repository
            .get_all_stacks()
            .map_err(|e| e.to_string())?;

        Ok(stacks
            .into_iter()
            .map(|s| StackResponseDto {
                id: s.id,
                nama_stack: s.nama_stack,
            })
            .collect())
    }
}
