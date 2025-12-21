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
    id: i32,
    name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name =tags)]
pub struct NewTag {
    name: String,
}

#[derive(Identifiable, Queryable, Selectable, Associations)]
#[diesel(table_name =blog)]
#[diesel(belongs_to(Category,foreign_key = category_id))]
pub struct Blog {
    id: i32,
    title: String,
    content: String,
    category_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name =blog)]
pub struct NewBlog {
    title: String,
    content: String,
    category_id: i32,
}

#[derive(Identifiable, Queryable, Selectable, Associations, Insertable)]
#[diesel(table_name =blog_tags)]
#[diesel(belongs_to(Blog,foreign_key = blog_id))]
#[diesel(belongs_to(Tag,foreign_key = tag_id))]
#[diesel(primary_key(blog_id, tag_id))]
pub struct BlogTags {
    blog_id: i32,
    tag_id: i32,
}
