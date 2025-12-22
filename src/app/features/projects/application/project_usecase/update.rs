use crate::app::features::projects::domain::entity::NewProject;
use crate::app::features::projects::domain::repository::ProjectRepository;
use crate::app::features::projects::interface::dto::UpdateProjectRequestDto;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn ProjectRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn ProjectRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, id: i32, dto: UpdateProjectRequestDto) -> Result<(), String> {
        let existing = self
            .repository
            .get_project_by_id(id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Project not found".to_string())?;

        let tanggal_mulai = match dto.tanggal_mulai {
            Some(d) => chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d")
                .map_err(|_| "Invalid start date format (YYYY-MM-DD)".to_string())?,
            None => existing.tanggal_mulai,
        };

        let tanggal_selesai = match dto.tanggal_selesai {
            Some(d) => Some(
                chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d")
                    .map_err(|_| "Invalid end date format (YYYY-MM-DD)".to_string())?,
            ),
            None => existing.tanggal_selesai,
        };

        let new_project = NewProject {
            nama_projek: dto.nama_projek.unwrap_or(existing.nama_projek),
            deskripsi: dto.deskripsi.unwrap_or(existing.deskripsi),
            status: dto.status.unwrap_or(existing.status),
            progress: dto.progress.unwrap_or(existing.progress),
            link_demo: dto.link_demo.or(existing.link_demo),
            repository: dto.repository.or(existing.repository),
            tanggal_mulai,
            tanggal_selesai,
        };

        self.repository
            .update_project(id, new_project)
            .map_err(|e| e.to_string())?;

        if let Some(stack_ids) = dto.stack_ids {
            // Replace all stacks
            self.repository
                .remove_all_stacks_from_project(id)
                .map_err(|e| e.to_string())?;

            for stack_id in stack_ids {
                self.repository
                    .add_stack_to_project(id, stack_id)
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }
}
