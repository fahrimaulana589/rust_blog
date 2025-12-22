use crate::app::features::projects::domain::repository::ProjectRepository;
use crate::app::features::projects::interface::dto::{ProjectResponseDto, StackResponseDto};
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn ProjectRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn ProjectRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, id: i32) -> Result<ProjectResponseDto, String> {
        let project = self
            .repository
            .get_project_by_id(id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Project not found".to_string())?;

        let stacks = self
            .repository
            .get_stacks_by_project_id(id)
            .map_err(|e| e.to_string())?;

        let stack_dtos = stacks
            .into_iter()
            .map(|s| StackResponseDto {
                id: s.id,
                nama_stack: s.nama_stack,
            })
            .collect();

        Ok(ProjectResponseDto {
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
        })
    }
}
