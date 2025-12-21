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

    pub async fn execute(&self, id: i32) -> Result<BlogResponseDto, String> {
        // Fetch Blog
        let blog = self
            .repository
            .get_blog_by_id(id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Blog not found".to_string())?;

        // Fetch Category
        let category = self
            .repository
            .get_category_by_id(blog.category_id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Category not found (Data Inconsistency)".to_string())?;

        // Fetch Tags
        let tags = self
            .repository
            .get_tags_by_blog_id(blog.id)
            .map_err(|e| e.to_string())?;

        // Assemble DTO
        Ok(BlogResponseDto {
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
        })
    }
}
