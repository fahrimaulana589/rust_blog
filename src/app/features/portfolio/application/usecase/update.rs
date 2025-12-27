use crate::app::features::portfolio::domain::entity::NewPortfolio;
use crate::app::features::portfolio::domain::error::PortfolioError;
use crate::app::features::portfolio::domain::repository::PortfolioRepository;
use crate::app::features::portfolio::interface::dto::{
    PortfolioResponseDto, UpdatePortfolioRequestDto,
};
use crate::app::features::projects::interface::dto::ProjectResponseDto;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn PortfolioRepository>,
}

impl Execute {
    pub fn new(repository: Arc<dyn PortfolioRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(
        &self,
        id: i32,
        dto: UpdatePortfolioRequestDto,
    ) -> Result<PortfolioResponseDto, PortfolioError> {
        // Fetch existing
        let (_existing, _, _) = self.repository.find_by_id(id).map_err(|e| {
            if e.to_string().to_lowercase().contains("not found") {
                PortfolioError::NotFound("Portfolio not found".to_string())
            } else {
                PortfolioError::System(e.to_string())
            }
        })?;

        use validator::{ValidationError, ValidationErrors};

        let mut validation_errors = ValidationErrors::new();

        if let Some(existing_portfolio) = self
            .repository
            .find_by_judul(dto.judul.clone())
            .map_err(|e| PortfolioError::System(e.to_string()))?
        {
            if existing_portfolio.id != id {
                validation_errors.add(
                    "judul",
                    ValidationError::new("Portfolio title already exists"),
                );
            }
        }

        if !validation_errors.is_empty() {
            return Err(PortfolioError::Validation(validation_errors));
        }

        // Merge logic
        let new_data = NewPortfolio {
            project_id: dto.project_id,
            judul: dto.judul,
            deskripsi: Some(dto.deskripsi),
            is_active: dto.is_active,
        };

        let (portfolio, project, stacks) = self
            .repository
            .update(id, new_data)
            .map_err(|e| PortfolioError::System(e.to_string()))?;

        Ok(PortfolioResponseDto {
            id: portfolio.id,
            judul: portfolio.judul,
            deskripsi: portfolio.deskripsi,
            is_active: portfolio.is_active,
            created_at: portfolio.created_at.to_string(),
            updated_at: portfolio.updated_at.to_string(),
            project: ProjectResponseDto {
                id: project.id,
                nama_projek: project.nama_projek,
                deskripsi: project.deskripsi,
                status: project.status,
                progress: project.progress,
                link_demo: project.link_demo,
                repository: project.repository,
                tanggal_mulai: project.tanggal_mulai.to_string(),
                tanggal_selesai: project.tanggal_selesai.map(|d| d.to_string()),
                stacks: stacks
                    .into_iter()
                    .map(
                        |s| crate::app::features::projects::interface::dto::StackResponseDto {
                            id: s.id,
                            nama_stack: s.nama_stack,
                        },
                    )
                    .collect(),
                created_at: project.created_at.to_string(),
                updated_at: project.updated_at.to_string(),
            },
        })
    }
}
