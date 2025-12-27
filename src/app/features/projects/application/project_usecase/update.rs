use crate::app::features::projects::domain::entity::NewProject;
use crate::app::features::projects::domain::error::ProjectError;
use crate::app::features::projects::domain::repository::ProjectRepository;
use crate::app::features::projects::interface::dto::UpdateProjectRequestDto;
use std::sync::Arc;
use validator::{Validate, ValidationError, ValidationErrors};

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
        id: i32,
        dto: UpdateProjectRequestDto,
    ) -> Result<crate::app::features::projects::interface::dto::ProjectResponseDto, ProjectError>
    {
        use crate::app::features::projects::interface::dto::{
            ProjectResponseDto, StackResponseDto,
        };

        let existing = self
            .repository
            .get_project_by_id(id)
            .map_err(|e| ProjectError::System(e.to_string()))?
            .ok_or_else(|| ProjectError::NotFound("Project not found".to_string()))?;

        let mut validation_errors = match dto.validate() {
            Ok(_) => ValidationErrors::new(),
            Err(e) => e,
        };

        if let Some(existing_proj) = self
            .repository
            .get_project_by_name(&dto.nama_projek)
            .map_err(|e| ProjectError::System(e.to_string()))?
        {
            if existing_proj.id != id {
                validation_errors.add(
                    "nama_projek",
                    ValidationError::new("Project name already exists"),
                );
            }
        }

        if !validation_errors.is_empty() {
            return Err(ProjectError::Validation(validation_errors));
        }

        let tanggal_mulai = chrono::NaiveDate::parse_from_str(&dto.tanggal_mulai, "%Y-%m-%d")
            .map_err(|_| {
                ProjectError::System("Invalid start date format (YYYY-MM-DD)".to_string())
            })?;

        let tanggal_selesai =
            match dto.tanggal_selesai {
                Some(d) => Some(chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").map_err(
                    |_| ProjectError::System("Invalid end date format (YYYY-MM-DD)".to_string()),
                )?),
                None => existing.tanggal_selesai,
            };

        let new_project = NewProject {
            nama_projek: dto.nama_projek,
            deskripsi: dto.deskripsi,
            status: dto.status,
            progress: dto.progress.unwrap_or(existing.progress),
            link_demo: dto.link_demo.or(existing.link_demo),
            repository: dto.repository.or(existing.repository),
            tanggal_mulai,
            tanggal_selesai,
        };

        let updated_project = self
            .repository
            .update_project(id, new_project)
            .map_err(|e| ProjectError::System(e.to_string()))?;

        if let Some(stack_ids) = dto.stack_ids {
            // Replace all stacks
            self.repository
                .remove_all_stacks_from_project(id)
                .map_err(|e| ProjectError::System(e.to_string()))?;

            for stack_id in stack_ids {
                self.repository
                    .add_stack_to_project(id, stack_id)
                    .map_err(|e| ProjectError::System(e.to_string()))?;
            }
        }

        // Fetch related stacks for response
        let stacks = self
            .repository
            .get_stacks_by_project_id(id)
            .map_err(|e| ProjectError::System(e.to_string()))?;

        let stack_dtos = stacks
            .into_iter()
            .map(|s| StackResponseDto {
                id: s.id,
                nama_stack: s.nama_stack,
            })
            .collect();

        Ok(ProjectResponseDto {
            id: updated_project.id,
            nama_projek: updated_project.nama_projek,
            deskripsi: updated_project.deskripsi,
            status: updated_project.status,
            progress: updated_project.progress,
            link_demo: updated_project.link_demo,
            repository: updated_project.repository,
            tanggal_mulai: updated_project.tanggal_mulai.to_string(),
            tanggal_selesai: updated_project.tanggal_selesai.map(|d| d.to_string()),
            stacks: stack_dtos,
            created_at: updated_project.created_at.to_string(),
            updated_at: updated_project.updated_at.to_string(),
        })
    }
}
