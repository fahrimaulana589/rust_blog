use crate::app::features::projects::domain::entity::NewProject;
use crate::app::features::projects::domain::repository::ProjectRepository;
use crate::app::features::projects::interface::dto::{
    CreateProjectRequestDto, ProjectResponseDto, StackResponseDto,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn ProjectRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn ProjectRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, dto: CreateProjectRequestDto) -> Result<ProjectResponseDto, String> {
        // Parse dates
        let tanggal_mulai = chrono::NaiveDate::parse_from_str(&dto.tanggal_mulai, "%Y-%m-%d")
            .map_err(|_| "Invalid start date format (YYYY-MM-DD)".to_string())?;

        let tanggal_selesai = match dto.tanggal_selesai {
            Some(d) => Some(
                chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d")
                    .map_err(|_| "Invalid end date format (YYYY-MM-DD)".to_string())?,
            ),
            None => None,
        };

        let new_project = NewProject {
            nama_projek: dto.nama_projek,
            deskripsi: dto.deskripsi,
            status: dto.status,
            progress: dto.progress,
            link_demo: dto.link_demo,
            repository: dto.repository,
            tanggal_mulai,
            tanggal_selesai,
        };

        // Transaction logic should be here ideally, but for now we do sequential insert
        let created_project = self
            .repository
            .create_project(new_project)
            .map_err(|e| e.to_string())?;

        // Add stacks
        let mut stack_dtos = Vec::new();
        if let Some(stack_ids) = dto.stack_ids {
            for stack_id in stack_ids {
                self.repository
                    .add_stack_to_project(created_project.id, stack_id)
                    .map_err(|e| e.to_string())?;

                if let Some(stack) = self
                    .repository
                    .get_stack_by_id(stack_id)
                    .map_err(|e| e.to_string())?
                {
                    stack_dtos.push(StackResponseDto {
                        id: stack.id,
                        nama_stack: stack.nama_stack,
                    });
                }
            }
        }

        Ok(ProjectResponseDto {
            id: created_project.id,
            nama_projek: created_project.nama_projek,
            deskripsi: created_project.deskripsi,
            status: created_project.status,
            progress: created_project.progress,
            link_demo: created_project.link_demo,
            repository: created_project.repository,
            tanggal_mulai: created_project.tanggal_mulai.to_string(),
            tanggal_selesai: created_project.tanggal_selesai.map(|d| d.to_string()),
            stacks: stack_dtos,
            created_at: created_project.created_at.to_string(),
            updated_at: created_project.updated_at.to_string(),
        })
    }
}
