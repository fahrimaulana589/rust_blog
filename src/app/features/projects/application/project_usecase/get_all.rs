use crate::app::features::projects::domain::repository::ProjectRepository;
use crate::app::features::projects::interface::dto::{
    MetaDto, PaginatedResponseDto, ProjectResponseDto, StackResponseDto,
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

    pub fn execute(
        &self,
        page: i64,
        per_page: i64,
    ) -> Result<PaginatedResponseDto<ProjectResponseDto>, String> {
        let offset = (page - 1) * per_page;
        let (projects, total_count) = self
            .repository
            .get_all_projects(per_page, offset)
            .map_err(|e| e.to_string())?;

        let mut project_dtos = Vec::new();
        for project in projects {
            let stacks = self
                .repository
                .get_stacks_by_project_id(project.id)
                .map_err(|e| e.to_string())?;

            let stack_dtos = stacks
                .into_iter()
                .map(|s| StackResponseDto {
                    id: s.id,
                    nama_stack: s.nama_stack,
                })
                .collect();

            project_dtos.push(ProjectResponseDto {
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
            });
        }

        let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;

        Ok(PaginatedResponseDto {
            items: project_dtos,
            meta: MetaDto {
                page,
                per_page,
                total_pages,
                total_items: total_count,
            },
        })
    }
}
