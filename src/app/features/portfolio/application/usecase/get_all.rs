use crate::app::features::portfolio::domain::repository::PortfolioRepository;
use crate::app::features::portfolio::interface::dto::{
    MetaDto, PaginatedResponseDto, PaginationRequestDto, PortfolioResponseDto,
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
        query: PaginationRequestDto,
    ) -> Result<PaginatedResponseDto<PortfolioResponseDto>, String> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(10);
        let offset = (page - 1) * per_page;

        let (items, total_count) = self
            .repository
            .find_all(offset, per_page)
            .map_err(|e| e.to_string())?;

        let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;

        let response_items: Vec<PortfolioResponseDto> = items
            .into_iter()
            .map(|(item, project, stacks)| PortfolioResponseDto {
                id: item.id,
                judul: item.judul,
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
