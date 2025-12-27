use crate::app::features::portfolio::domain::entity::NewPortfolio;
use crate::app::features::portfolio::domain::error::PortfolioError;
use crate::app::features::portfolio::domain::repository::PortfolioRepository;
use crate::app::features::portfolio::interface::dto::UpdatePortfolioRequestDto;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn PortfolioRepository>,
}

impl Execute {
    pub fn new(repository: Arc<dyn PortfolioRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, id: i32, dto: UpdatePortfolioRequestDto) -> Result<(), PortfolioError> {
        // Fetch existing
        let (_existing, _, _) = self.repository.find_by_id(id).map_err(|e| {
            if e.to_string().to_lowercase().contains("not found") {
                PortfolioError::NotFound("Portfolio not found".to_string())
            } else {
                PortfolioError::System(e.to_string())
            }
        })?;

        // Merge logic
        let new_data = NewPortfolio {
            project_id: dto.project_id,
            judul: dto.judul,
            deskripsi: Some(dto.deskripsi),
            is_active: dto.is_active,
        };

        self.repository
            .update(id, new_data)
            .map_err(|e| PortfolioError::System(e.to_string()))?;

        Ok(())
    }
}
