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
        // Delete blog tags first (manual cascade if specific DB setup doesn't do it)
        // Usually safer to do it.
        self.repository
            .delete_blog_tags_by_blog_id(id)
            .map_err(|e| e.to_string())?;

        self.repository.delete_blog(id).map_err(|e| e.to_string())?;
        Ok(())
    }
}
