use chrono::Utc;

use crate::app::features::blog::domain::entity::{BlogTags, NewBlog};
use crate::app::features::blog::domain::repository::BlogRepository;
use crate::app::features::blog::interface::dto::CreateBlogRequestDto;
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
        dto: CreateBlogRequestDto,
    ) -> Result<
        BlogResponseDto,
        BlogError,
    > {
        

        let mut validation_errors = dto.validate().err().unwrap_or_default();

        // Validate Category Exists and Fetch it
        let category = match self
            .repository
            .get_category_by_id(dto.category_id)
            .map_err(|e| BlogError::System(e.to_string()))?
        {
            Some(c) => Some(c),
            None => {
                validation_errors.add("category_id", ValidationError::new("Category not found"));
                None
            }
        };

        let val_tag_ids = dto.tag_ids.clone();
        let mut tags = Vec::new();

        // Validate Tag Exists and Fetch them
        if let Some(tag_ids) = val_tag_ids {
            for tag_id in tag_ids {
                match self
                    .repository
                    .get_tag_by_id(tag_id)
                    .map_err(|e| BlogError::System(e.to_string()))?
                {
                    Some(t) => tags.push(t),
                    None => {
                        validation_errors.add("tag_ids", ValidationError::new("Tag not found"));
                        break;
                    }
                }
            }
        }

        // Create Blog
        let slug = dto
            .title
            .to_lowercase()
            .replace(" ", "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>();

        // Validate Title/Slug Uniqueness
        if self
            .repository
            .get_blog_by_slug(slug.clone())
            .map_err(|e| BlogError::System(e.to_string()))?
            .is_some()
        {
            validation_errors.add("title", ValidationError::new("Title already exists"));
        }

        if !validation_errors.is_empty() {
            return Err(BlogError::Validation(validation_errors));
        }

        // Create Blog
        let status = dto.status;

        let published_at = if status == "PUBLISHED" {
            Some(Utc::now().naive_utc())
        } else {
            None
        };

        let new_blog = NewBlog {
            title: dto.title,
            content: dto.content,
            category_id: dto.category_id,
            slug: slug.clone(),
            excerpt: dto.excerpt,
            thumbnail: dto.thumbnail,
            status: status.clone(),
            published_at,
            view_count: 0,
        };
        let created_blog = self
            .repository
            .create_blog(new_blog)
            .map_err(|e| BlogError::System(e.to_string()))?;

        // Create Blog Tags if provided
        // We already have tags fetched, we just need to link them
        for tag in &tags {
            let blog_tags = BlogTags {
                blog_id: created_blog.id,
                tag_id: tag.id,
            };
            self.repository
                .create_blog_tags(blog_tags)
                .map_err(|e| BlogError::System(e.to_string()))?;
        }

        // Construct Response
        let category = category.unwrap(); // Safe because validation passed
        let category_dto = CategoryResponseDto {
            id: category.id,
            name: category.name,
            created_at: category.created_at.to_string(),
            updated_at: category.updated_at.to_string(),
        };

        let tags_dto = tags
            .into_iter()
            .map(|t| TagResponseDto {
                id: t.id,
                name: t.name,
                created_at: t.created_at.to_string(),
                updated_at: t.updated_at.to_string(),
            })
            .collect();

        Ok(BlogResponseDto {
            id: created_blog.id,
            title: created_blog.title,
            slug: created_blog.slug,
            content: created_blog.content,
            excerpt: created_blog.excerpt,
            thumbnail: created_blog.thumbnail,
            status: created_blog.status,
            view_count: created_blog.view_count,
            category: category_dto,
            tags: tags_dto,
            created_at: created_blog.created_at.to_string(),
            updated_at: created_blog.updated_at.to_string(),
            published_at: created_blog.published_at.map(|d| d.to_string()),
        })
    }
}
