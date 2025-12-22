use crate::app::features::portfolio::domain::entity::NewPortfolio;
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
    ) -> Result<PortfolioResponseDto, String> {
        // Fetch existing
        let (existing, _, _) = self.repository.find_by_id(id).map_err(|e| e.to_string())?;

        // Merge logic
        let new_data = NewPortfolio {
            project_id: dto.project_id.unwrap_or(existing.project_id),
            judul: dto.judul.unwrap_or(existing.judul),
            deskripsi: dto.deskripsi.or(existing.deskripsi),
            is_active: dto.is_active.unwrap_or(existing.is_active),
        };

        let (result, result_project, stacks) = self
            .repository
            .update(id, new_data)
            .map_err(|e| e.to_string())?;

        Ok(PortfolioResponseDto {
            id: result.id,
            judul: result.judul,
            deskripsi: result.deskripsi,
            is_active: result.is_active,
            created_at: result.created_at.to_string(),
            updated_at: result.updated_at.to_string(),
            project: ProjectResponseDto {
                id: result_project.id,
                nama_projek: result_project.nama_projek,
                deskripsi: result_project.deskripsi,
                status: result_project.status,
                progress: result_project.progress,
                link_demo: result_project.link_demo,
                repository: result_project.repository,
                tanggal_mulai: result_project.tanggal_mulai.to_string(),
                tanggal_selesai: result_project.tanggal_selesai.map(|d| d.to_string()),
                stacks: stacks
                    .into_iter()
                    .map(
                        |s| crate::app::features::projects::interface::dto::StackResponseDto {
                            id: s.id,
                            nama_stack: s.nama_stack,
                        },
                    )
                    .collect(),
                created_at: result_project.created_at.to_string(),
                updated_at: result_project.updated_at.to_string(),
            },
        })
    }
}
