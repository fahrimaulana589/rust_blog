use crate::app::features::blog::domain::entity::{BlogTags, NewBlog};
use crate::app::features::blog::domain::repository::BlogRepository;
use crate::app::features::blog::interface::dto::UpdateBlogRequestDto;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    repository: Arc<dyn BlogRepository + Send + Sync>,
}

impl Execute {
    pub fn new(repository: Arc<dyn BlogRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: i32, dto: UpdateBlogRequestDto) -> Result<(), String> {
        // Check Existence
        let existing_blog = self
            .repository
            .get_blog_by_id(id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Blog not found".to_string())?;

        // Validate Category
        if let Some(cat_id) = dto.category_id {
            if self
                .repository
                .get_category_by_id(cat_id)
                .map_err(|e| e.to_string())?
                .is_none()
            {
                return Err("Category not found".to_string());
            }
        }

        // Update Blog (Merge DTO with existing)
        let new_blog = NewBlog {
            title: dto.title.unwrap_or(existing_blog.title),
            content: dto.content.unwrap_or(existing_blog.content),
            category_id: dto.category_id.unwrap_or(existing_blog.category_id),
            slug: existing_blog.slug, // Preserve slug
            excerpt: dto.excerpt.or(existing_blog.excerpt),
            thumbnail: dto.thumbnail.or(existing_blog.thumbnail),
            status: dto.status.unwrap_or(existing_blog.status),
            published_at: existing_blog.published_at, // Preserve published_at
            view_count: existing_blog.view_count,     // Preserve view_count
        };
        self.repository
            .update_blog(id, new_blog)
            .map_err(|e| e.to_string())?;

        // Update Tags if provided
        if let Some(tag_ids) = dto.tag_ids {
            // Delete existing tags
            self.repository
                .delete_blog_tags_by_blog_id(id)
                .map_err(|e| e.to_string())?;

            // Add new tags
            for tag_id in tag_ids {
                let blog_tags = BlogTags {
                    blog_id: id,
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
