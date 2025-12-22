use crate::app::features::portfolio::domain::repository::PortfolioRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn PortfolioRepository>,
}

impl Execute {
    pub fn new(repository: Arc<dyn PortfolioRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, id: i32) -> Result<(), String> {
        let count = self.repository.delete(id).map_err(|e| e.to_string())?;
        if count == 0 {
            return Err("Portfolio not found".to_string());
        }
        Ok(())
    }
}
