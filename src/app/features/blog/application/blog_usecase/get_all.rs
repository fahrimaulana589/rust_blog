use crate::app::features::blog::domain::repository::BlogRepository;
use crate::app::features::blog::interface::dto::{
    BlogResponseDto, CategoryResponseDto, TagResponseDto,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn BlogRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn BlogRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        query: crate::app::features::blog::interface::dto::PaginationRequestDto,
    ) -> Result<
        crate::app::features::blog::interface::dto::PaginatedResponseDto<BlogResponseDto>,
        String,
    > {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(10);
        let offset = (page - 1) * per_page;

        let (blogs, total_items) = self
            .repository
            .get_all_blog(per_page, offset)
            .map_err(|e| e.to_string())?;

        let mut dtos = Vec::new();

        for blog in blogs {
            let category = self
                .repository
                .get_category_by_id(blog.category_id)
                .map_err(|e| e.to_string())?
                .ok_or_else(|| "Category not found for blog".to_string())?;

            let tags = self
                .repository
                .get_tags_by_blog_id(blog.id)
                .map_err(|e| e.to_string())?;

            dtos.push(BlogResponseDto {
                id: blog.id,
                slug: blog.slug,
                title: blog.title,
                content: blog.content,
                excerpt: blog.excerpt,
                thumbnail: blog.thumbnail,
                status: blog.status,
                view_count: blog.view_count,
                category: CategoryResponseDto {
                    id: category.id,
                    name: category.name,
                    created_at: category.created_at.to_string(),
                    updated_at: category.updated_at.to_string(),
                },
                tags: tags
                    .into_iter()
                    .map(|t| TagResponseDto {
                        id: t.id,
                        name: t.name,
                        created_at: t.created_at.to_string(),
                        updated_at: t.updated_at.to_string(),
                    })
                    .collect(),
                created_at: blog.created_at.to_string(),
                updated_at: blog.updated_at.to_string(),
                published_at: blog.published_at.map(|t| t.to_string()),
            });
        }

        let total_pages = (total_items as f64 / per_page as f64).ceil() as i64;

        Ok(
            crate::app::features::blog::interface::dto::PaginatedResponseDto {
                items: dtos,
                meta: crate::app::features::blog::interface::dto::MetaDto {
                    page,
                    per_page,
                    total_pages,
                    total_items,
                },
            },
        )
    }
}
