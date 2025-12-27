use crate::app::features::portofolio::domain::error::PortofolioError;
use crate::app::features::portofolio::domain::repository::PortofolioRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn PortofolioRepository>,
}

impl Execute {
    pub fn new(repository: Arc<dyn PortofolioRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, id: i32) -> Result<(), PortofolioError> {
        let count = self
            .repository
            .delete(id)
            .map_err(|e| PortofolioError::System(e.to_string()))?;
        if count == 0 {
            return Err(PortofolioError::NotFound(
                "Portofolio not found".to_string(),
            ));
        }
        Ok(())
    }
}
