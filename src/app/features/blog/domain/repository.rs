use crate::app::features::blog::domain::entity::{
    Blog, BlogTags, Category, NewBlog, NewCategory, NewTag, Tag,
};
use diesel::QueryResult;

pub trait BlogRepository {
    fn get_all_blog(&self) -> QueryResult<Vec<Blog>>;
    fn get_blog_by_id(&self, id: i32) -> QueryResult<Option<Blog>>;
    fn create_blog(&self, blog: NewBlog) -> QueryResult<Blog>;
    fn create_blog_tags(&self, blog_tags: BlogTags) -> QueryResult<BlogTags>;
    fn update_blog(&self, id: i32, blog: NewBlog) -> QueryResult<Blog>;
    fn delete_blog(&self, id: i32) -> QueryResult<usize>;
    fn delete_blog_tags_by_blog_id(&self, blog_id: i32) -> QueryResult<usize>;
    fn get_tags_by_blog_id(&self, blog_id: i32) -> QueryResult<Vec<Tag>>;
    fn get_all_tag(&self) -> QueryResult<Vec<Tag>>;
    fn get_tag_by_id(&self, id: i32) -> QueryResult<Option<Tag>>;
    fn create_tag(&self, tag: NewTag) -> QueryResult<Tag>;
    fn update_tag(&self, id: i32, tag: NewTag) -> QueryResult<Tag>;
    fn delete_tag(&self, id: i32) -> QueryResult<usize>;
    fn get_all_category(&self) -> QueryResult<Vec<Category>>;
    fn get_category_by_id(&self, id: i32) -> QueryResult<Option<Category>>;
    fn create_category(&self, category: NewCategory) -> QueryResult<Category>;
    fn update_category(&self, id: i32, category: NewCategory) -> QueryResult<Category>;
    fn delete_category(&self, id: i32) -> QueryResult<usize>;
}
