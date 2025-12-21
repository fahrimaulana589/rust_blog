use crate::app::features::blog::domain::entity::{BlogTags, NewBlog};
use crate::app::features::blog::domain::repository::BlogRepository;
use crate::app::features::blog::interface::dto::CreateBlogRequestDto;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn BlogRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn BlogRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, dto: CreateBlogRequestDto) -> Result<(), String> {
        // Validate Category Exists
        if self
            .repository
            .get_category_by_id(dto.category_id)
            .map_err(|e| e.to_string())?
            .is_none()
        {
            return Err("Category not found".to_string());
        }

        // Create Blog
        let new_blog = NewBlog {
            title: dto.title,
            content: dto.content,
            category_id: dto.category_id,
        };
        let created_blog = self
            .repository
            .create_blog(new_blog)
            .map_err(|e| e.to_string())?;

        // Create Blog Tags if provided
        if let Some(tag_ids) = dto.tag_ids {
            for tag_id in tag_ids {
                // Validate Tag Exists? Or rely on FK?
                // Let's rely on FK for tags to keep it snappy, or strict.
                // Strict: checks each. FK: simpler.
                // Let's do FK but if it fails it might be obscure.
                // For now, straight insert.
                let blog_tags = BlogTags {
                    blog_id: created_blog.id,
                    tag_id,
                };
                self.repository
                    .create_blog_tags(blog_tags)
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }
}
