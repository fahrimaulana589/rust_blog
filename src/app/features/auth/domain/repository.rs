use crate::app::features::auth::domain::entity::User;
use diesel::QueryResult;

pub trait UserRepository {
    fn get(&self, id: &i32) -> QueryResult<Option<User>>;
    fn get_where(&self, username: String, password: String) -> QueryResult<Option<User>>;
    fn create(&self, username: String, email: String, password: String) -> QueryResult<User>;
    fn reset_password(&self, username: String, password: String) -> QueryResult<User>;
}
