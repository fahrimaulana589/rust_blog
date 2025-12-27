use crate::app::features::portofolio::domain::error::PortofolioError;
use crate::app::features::portofolio::domain::repository::PortofolioRepository;
use crate::app::features::portofolio::interface::dto::PortofolioResponseDto;
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

    pub fn execute(&self, id: i32) -> Result<PortofolioResponseDto, PortofolioError> {
        let result = self.repository.find_by_id(id).map_err(|e| {
            if e.to_string().to_lowercase().contains("not found") {
                PortofolioError::NotFound("Portofolio not found".to_string())
            } else {
                PortofolioError::System(e.to_string())
            }
        })?;

        Ok(PortofolioResponseDto {
            id: result.0.id,
            judul: result.0.judul,
            deskripsi: result.0.deskripsi,
            is_active: result.0.is_active,
            created_at: result.0.created_at.to_string(),
            updated_at: result.0.updated_at.to_string(),
            project: ProjectResponseDto {
                id: result.1.id,
                nama_projek: result.1.nama_projek,
                deskripsi: result.1.deskripsi,
                status: result.1.status,
                progress: result.1.progress,
                link_demo: result.1.link_demo,
                repository: result.1.repository,
                tanggal_mulai: result.1.tanggal_mulai.to_string(),
                tanggal_selesai: result.1.tanggal_selesai.map(|d| d.to_string()),
                stacks: result
                    .2
                    .into_iter()
                    .map(
                        |s| crate::app::features::projects::interface::dto::StackResponseDto {
                            id: s.id,
                            nama_stack: s.nama_stack,
                        },
                    )
                    .collect(),
                created_at: result.1.created_at.to_string(),
                updated_at: result.1.updated_at.to_string(),
            },
        })
    }
}
