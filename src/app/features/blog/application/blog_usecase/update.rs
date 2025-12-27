use chrono::Utc;

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

    pub async fn execute(
        &self,
        id: i32,
        dto: UpdateBlogRequestDto,
    ) -> Result<(), crate::app::features::blog::domain::error::BlogError> {
        use crate::app::features::blog::domain::error::BlogError;
        use validator::Validate;
        use validator::ValidationError;

        // Check Existence
        let existing_blog = self
            .repository
            .get_blog_by_id(id)
            .map_err(|e| BlogError::System(e.to_string()))?
            .ok_or_else(|| BlogError::NotFound("Blog not found".to_string()))?;

        let mut validation_errors = dto.validate().err().unwrap_or_default();

        // Validate Category Exists
        if self
            .repository
            .get_category_by_id(dto.category_id)
            .map_err(|e| BlogError::System(e.to_string()))?
            .is_none()
        {
            validation_errors.add("category_id", ValidationError::new("Category not found"));
        }

        let val_tag_ids = dto.tag_ids.clone();

        // Validate Tag Exists
        if let Some(tag_ids) = val_tag_ids {
            for tag_id in tag_ids {
                if self
                    .repository
                    .get_tag_by_id(tag_id)
                    .map_err(|e| BlogError::System(e.to_string()))?
                    .is_none()
                {
                    validation_errors.add("tag_ids", ValidationError::new("Tag not found"));
                    break;
                }
            }
        }

        let title = dto.title;

        let slug = title
            .to_lowercase()
            .replace(" ", "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>();

        // Validate Title/Slug Uniqueness
        if let Some(existing) = self
            .repository
            .get_blog_by_slug(slug.clone())
            .map_err(|e| BlogError::System(e.to_string()))?
        {
            if existing.id != id {
                validation_errors.add("title", ValidationError::new("Title already exists"));
            }
        }

        if !validation_errors.is_empty() {
            return Err(BlogError::Validation(validation_errors));
        }

        let status = dto.status;

        let published_at = if status == "PUBLISHED" {
            Some(Utc::now().naive_utc())
        } else {
            None
        };

        // Update Blog (Merge DTO with existing)
        let new_blog = NewBlog {
            title,
            content: dto.content,
            category_id: dto.category_id,
            slug,
            excerpt: dto.excerpt,
            thumbnail: dto.thumbnail.or(existing_blog.thumbnail),
            status,
            published_at,
            view_count: existing_blog.view_count,
        };
        self.repository
            .update_blog(id, new_blog)
            .map_err(|e| BlogError::System(e.to_string()))?;

        // Update Tags if provided
        if let Some(tag_ids) = dto.tag_ids {
            // Delete existing tags
            self.repository
                .delete_blog_tags_by_blog_id(id)
                .map_err(|e| BlogError::System(e.to_string()))?;

            // Add new tags
            for tag_id in tag_ids {
                let blog_tags = BlogTags {
                    blog_id: id,
                    tag_id,
                };
                self.repository
                    .create_blog_tags(blog_tags)
                    .map_err(|e| BlogError::System(e.to_string()))?;
            }
        }

        Ok(())
    }
}
