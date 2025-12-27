use crate::app::features::portofolio::domain::error::PortofolioError;
use crate::app::features::portofolio::domain::repository::PortofolioRepository;
use crate::app::features::portofolio::interface::dto::{
    MetaDto, PaginatedResponseDto, PaginationRequestDto, PortofolioResponseDto,
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
        query: PaginationRequestDto,
    ) -> Result<PaginatedResponseDto<PortofolioResponseDto>, PortofolioError> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(10);
        let offset = (page - 1) * per_page;

        let (items, total_count) = self
            .repository
            .find_all(offset, per_page)
            .map_err(|e| PortofolioError::System(e.to_string()))?;

        let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;

        let response_items: Vec<PortofolioResponseDto> = items
            .into_iter()
            .map(|(item, project, stacks)| PortofolioResponseDto {
                id: item.id,
                judul: item.judul,
                slug: item.slug,
                deskripsi: item.deskripsi,
                is_active: item.is_active,
                created_at: item.created_at.to_string(),
                updated_at: item.updated_at.to_string(),
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
                    slug: project.slug,
                },
            })
            .collect();

        Ok(PaginatedResponseDto {
            items: response_items,
            meta: MetaDto {
                page,
                per_page,
                total_pages,
                total_items: total_count,
            },
        })
    }
}
