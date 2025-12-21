use crate::app::features::blog::domain::repository::BlogRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn BlogRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn BlogRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: i32) -> Result<(), String> {
        self.repository.delete_tag(id).map_err(|e| e.to_string())?;
        Ok(())
    }
}
