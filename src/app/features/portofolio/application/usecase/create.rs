use crate::app::features::portofolio::domain::entity::NewPortofolio;
use crate::app::features::portofolio::domain::error::PortofolioError;
use crate::app::features::portofolio::domain::repository::PortofolioRepository;
use crate::app::features::portofolio::interface::dto::{
    CreatePortofolioRequestDto, PortofolioResponseDto,
};
use crate::app::features::projects::interface::dto::ProjectResponseDto;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn PortofolioRepository>,
}

impl Execute {
    pub fn new(repository: Arc<dyn PortofolioRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(
        &self,
        dto: CreatePortofolioRequestDto,
    ) -> Result<PortofolioResponseDto, PortofolioError> {
        use validator::{ValidationError, ValidationErrors};

        let mut validation_errors = ValidationErrors::new();

        if let Some(_existing) = self
            .repository
            .find_by_judul(dto.judul.clone())
            .map_err(|e| PortofolioError::System(e.to_string()))?
        {
            validation_errors.add(
                "judul",
                ValidationError::new("Portofolio title already exists"),
            );
        }

        if !validation_errors.is_empty() {
            return Err(PortofolioError::Validation(validation_errors));
        }

        let new_portfolio = NewPortofolio {
            project_id: dto.project_id,
            judul: dto.judul,
            deskripsi: Some(dto.deskripsi),
            is_active: dto.is_active,
        };

        let (portfolio, project, stacks) = self
            .repository
            .create(new_portfolio)
            .map_err(|e| PortofolioError::System(e.to_string()))?;

        Ok(PortofolioResponseDto {
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
