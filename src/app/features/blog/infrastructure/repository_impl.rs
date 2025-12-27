use crate::app::features::blog::domain::entity::{
    Blog, BlogTags, Category, NewBlog, NewCategory, NewTag, Tag,
};
use crate::app::features::blog::domain::repository::BlogRepository;
use crate::schema::{blog, blog_tags, categories, tags};
use crate::utils::db::DbPool;
use diesel::prelude::*;

#[derive(Clone)]
pub struct BlogRepositoryImpl {
    pub pool: DbPool,
}

impl BlogRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl BlogRepository for BlogRepositoryImpl {
    fn get_all_blog(&self, limit: i64, offset: i64) -> QueryResult<(Vec<Blog>, i64)> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");

        let count = blog::table.count().get_result(&mut conn)?;
        let items = blog::table
            .limit(limit)
            .offset(offset)
            .load::<Blog>(&mut conn)?;

        Ok((items, count))
    }
    fn get_blog_by_id(&self, id: i32) -> QueryResult<Option<Blog>> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        blog::table.find(id).first::<Blog>(&mut conn).optional()
    }
    fn create_blog(&self, blog: NewBlog) -> QueryResult<Blog> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        diesel::insert_into(blog::table)
            .values(blog)
            .get_result(&mut conn)
    }
    fn update_blog(&self, id: i32, blog: NewBlog) -> QueryResult<Blog> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        diesel::update(blog::table.find(id))
            .set(blog)
            .get_result(&mut conn)
    }
    fn delete_blog(&self, id: i32) -> QueryResult<usize> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        diesel::delete(blog::table.find(id)).execute(&mut conn)
    }
    fn delete_blog_tags_by_blog_id(&self, blog_id: i32) -> QueryResult<usize> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        diesel::delete(blog_tags::table.filter(blog_tags::blog_id.eq(blog_id))).execute(&mut conn)
    }
    fn get_tags_by_blog_id(&self, blog_id: i32) -> QueryResult<Vec<Tag>> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        blog_tags::table
            .filter(blog_tags::blog_id.eq(blog_id))
            .inner_join(tags::table)
            .select(tags::all_columns)
            .load::<Tag>(&mut conn)
    }
    fn get_all_tag(&self, limit: i64, offset: i64) -> QueryResult<(Vec<Tag>, i64)> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");

        let count = tags::table.count().get_result(&mut conn)?;
        let items = tags::table
            .limit(limit)
            .offset(offset)
            .load::<Tag>(&mut conn)?;

        Ok((items, count))
    }
    fn get_tag_by_id(&self, id: i32) -> QueryResult<Option<Tag>> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        tags::table.find(id).first::<Tag>(&mut conn).optional()
    }
    fn create_tag(&self, tag: NewTag) -> QueryResult<Tag> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        diesel::insert_into(tags::table)
            .values(tag)
            .get_result(&mut conn)
    }
    fn update_tag(&self, id: i32, tag: NewTag) -> QueryResult<Tag> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        diesel::update(tags::table.find(id))
            .set(tag)
            .get_result(&mut conn)
    }
    fn delete_tag(&self, id: i32) -> QueryResult<usize> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        diesel::delete(tags::table.find(id)).execute(&mut conn)
    }
    fn get_all_category(&self, limit: i64, offset: i64) -> QueryResult<(Vec<Category>, i64)> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");

        let count = categories::table.count().get_result(&mut conn)?;
        let items = categories::table
            .limit(limit)
            .offset(offset)
            .load::<Category>(&mut conn)?;

        Ok((items, count))
    }
    fn get_category_by_id(&self, id: i32) -> QueryResult<Option<Category>> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        categories::table
            .find(id)
            .first::<Category>(&mut conn)
            .optional()
    }
    fn create_category(&self, category: NewCategory) -> QueryResult<Category> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        diesel::insert_into(categories::table)
            .values(category)
            .get_result(&mut conn)
    }
    fn update_category(&self, id: i32, category: NewCategory) -> QueryResult<Category> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        diesel::update(categories::table.find(id))
            .set(category)
            .get_result(&mut conn)
    }
    fn delete_category(&self, id: i32) -> QueryResult<usize> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        diesel::delete(categories::table.find(id)).execute(&mut conn)
    }
    fn create_blog_tags(&self, blog_tags: BlogTags) -> QueryResult<BlogTags> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        diesel::insert_into(blog_tags::table)
            .values(blog_tags)
            .get_result(&mut conn)
    }

    fn get_blog_by_slug(&self, slug: String) -> QueryResult<Option<Blog>> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        blog::table
            .filter(blog::slug.eq(slug))
            .first::<Blog>(&mut conn)
            .optional()
    }
}
