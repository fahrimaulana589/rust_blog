use crate::schema::{blog, blog_tags, categories, tags};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name =categories)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name =categories)]
pub struct NewCategory {
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name =tags)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name =tags)]
pub struct NewTag {
    pub name: String,
}

#[derive(Identifiable, Queryable, Selectable, Associations)]
#[diesel(table_name =blog)]
#[diesel(belongs_to(Category,foreign_key = category_id))]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub category_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub slug: String,
    pub excerpt: Option<String>,
    pub thumbnail: Option<String>,
    pub status: String,
    pub published_at: Option<NaiveDateTime>,
    pub view_count: i32,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name =blog)]
pub struct NewBlog {
    pub title: String,
    pub content: String,
    pub category_id: i32,
    pub slug: String,
    pub excerpt: Option<String>,
    pub thumbnail: Option<String>,
    pub status: String,
    pub published_at: Option<NaiveDateTime>,
    pub view_count: i32,
}

#[derive(Identifiable, Queryable, Selectable, Associations, Insertable)]
#[diesel(table_name =blog_tags)]
#[diesel(belongs_to(Blog,foreign_key = blog_id))]
#[diesel(belongs_to(Tag,foreign_key = tag_id))]
#[diesel(primary_key(blog_id, tag_id))]
pub struct BlogTags {
    pub blog_id: i32,
    pub tag_id: i32,
}
