use chrono::Utc;

use crate::app::features::blog::domain::entity::{BlogTags, NewBlog};
use crate::app::features::blog::domain::repository::BlogRepository;
use crate::app::features::blog::interface::dto::UpdateBlogRequestDto;
use std::sync::Arc;
use crate::app::features::blog::domain::error::BlogError;
use crate::app::features::blog::interface::dto::{
    BlogResponseDto, CategoryResponseDto, TagResponseDto,
};
use validator::Validate;
use validator::ValidationError;

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
    ) -> Result<
        BlogResponseDto,
        BlogError,
    > {
        

        // Check Existence
        let existing_blog = self
            .repository
            .get_blog_by_id(id)
            .map_err(|e| BlogError::System(e.to_string()))?
            .ok_or_else(|| BlogError::NotFound("Blog not found".to_string()))?;

        let mut validation_errors = dto.validate().err().unwrap_or_default();

        // Re-implementing validation logic properly
        let mut new_category_obj = None;
        if self
            .repository
            .get_category_by_id(dto.category_id)
            .map_err(|e| BlogError::System(e.to_string()))?
            .map(|c| {
                new_category_obj = Some(c.clone());
                c
            })
            .is_none()
        {
            validation_errors.add("category_id", ValidationError::new("Category not found"));
        }

        let val_tag_ids = dto.tag_ids.clone();
        let mut new_tags_objs = Vec::new();

        // Validate Tag Exists
        if let Some(tag_ids) = val_tag_ids {
            for tag_id in tag_ids {
                match self
                    .repository
                    .get_tag_by_id(tag_id)
                    .map_err(|e| BlogError::System(e.to_string()))?
                {
                    Some(t) => new_tags_objs.push(t),
                    None => {
                        validation_errors.add("tag_ids", ValidationError::new("Tag not found"));
                        break;
                    }
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
            status: status.clone(),
            published_at,
            view_count: existing_blog.view_count,
        };
        let updated_blog = self
            .repository
            .update_blog(id, new_blog)
            .map_err(|e| BlogError::System(e.to_string()))?;

        // Update Tags if provided
        if let Some(ref tag_ids) = dto.tag_ids {
            // Delete existing tags
            self.repository
                .delete_blog_tags_by_blog_id(id)
                .map_err(|e| BlogError::System(e.to_string()))?;

            // Add new tags
            for tag_id in tag_ids {
                let blog_tags = BlogTags {
                    blog_id: id,
                    tag_id: *tag_id,
                };
                self.repository
                    .create_blog_tags(blog_tags)
                    .map_err(|e| BlogError::System(e.to_string()))?;
            }
        }

        // Construct Response Data
        // 1. Category
        let final_category = new_category_obj
            .ok_or_else(|| BlogError::System("Category should exist".to_string()))?; // We validated it exists above

        let category_dto = CategoryResponseDto {
            id: final_category.id,
            name: final_category.name,
            created_at: final_category.created_at.to_string(),
            updated_at: final_category.updated_at.to_string(),
        };

        // 2. Tags
        // If dto.tag_ids was Some, we have new_tags_objs populated.
        // If dto.tag_ids was None, we need to fetch existing tags (since we didn't touch them).
        let final_tags = if dto.tag_ids.is_some() {
            new_tags_objs
        } else {
            self.repository
                .get_tags_by_blog_id(id)
                .map_err(|e| BlogError::System(e.to_string()))?
        };

        let tags_dto = final_tags
            .into_iter()
            .map(|t| TagResponseDto {
                id: t.id,
                name: t.name,
                created_at: t.created_at.to_string(),
                updated_at: t.updated_at.to_string(),
            })
            .collect();

        Ok(BlogResponseDto {
            id: updated_blog.id,
            title: updated_blog.title,
            slug: updated_blog.slug,
            content: updated_blog.content,
            excerpt: updated_blog.excerpt,
            thumbnail: updated_blog.thumbnail,
            status: updated_blog.status,
            view_count: updated_blog.view_count,
            category: category_dto,
            tags: tags_dto,
            created_at: updated_blog.created_at.to_string(),
            updated_at: updated_blog.updated_at.to_string(),
            published_at: updated_blog.published_at.map(|d| d.to_string()),
        })
    }
}
