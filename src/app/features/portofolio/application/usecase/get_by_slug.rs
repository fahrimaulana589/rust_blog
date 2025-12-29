use crate::app::features::portofolio::domain::error::PortofolioError;
use crate::app::features::portofolio::domain::repository::PortofolioRepository;
use crate::app::features::portofolio::interface::dto::PortofolioResponseDto;
use crate::app::features::projects::interface::dto::{ProjectResponseDto, StackResponseDto};
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    pub portofolio_repository: Arc<dyn PortofolioRepository>,
}

impl Execute {
    pub fn new(portofolio_repository: Arc<dyn PortofolioRepository>) -> Self {
        Self {
            portofolio_repository,
        }
    }

    pub fn execute(&self, slug: String) -> Result<PortofolioResponseDto, PortofolioError> {
        let (portfolio, project, stacks) = self
            .portofolio_repository
            .find_by_slug(slug.clone())
            .map_err(|e| PortofolioError::System(e.to_string()))?
            .ok_or_else(|| {
                PortofolioError::NotFound(format!("Portofolio with slug {} not found", slug))
            })?;

        let stack_dtos = stacks
            .into_iter()
            .map(|s| StackResponseDto {
                id: s.id,
                nama_stack: s.nama_stack,
            })
            .collect();

        Ok(PortofolioResponseDto {
            id: portfolio.id,
            judul: portfolio.judul,
            slug: portfolio.slug,
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
                stacks: stack_dtos,
                created_at: project.created_at.to_string(),
                updated_at: project.updated_at.to_string(),
                slug: project.slug,
            },
        })
    }
}
