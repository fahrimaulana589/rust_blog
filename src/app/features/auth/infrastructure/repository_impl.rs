use crate::app::features::auth::domain::entity::User;
use crate::app::features::auth::domain::repository::UserRepository;
use crate::schema::users;
use crate::utils::db::DbPool;
use diesel::prelude::*;
use diesel::result::QueryResult;

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pub pool: DbPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

//implement repository
impl UserRepository for UserRepositoryImpl {
    fn get(&self, get_id: &i32) -> QueryResult<Option<User>> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        users::table
            .filter(users::id.eq(get_id))
            .first::<User>(&mut conn)
            .optional()
    }
    fn get_where(&self, name: String, pass: String) -> QueryResult<Option<User>> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        users::table
            .filter(users::username.eq(name))
            .filter(users::password.eq(pass))
            .first::<User>(&mut conn)
            .optional()
    }
    fn reset_password(&self, name: String, pass: String) -> QueryResult<User> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");

        diesel::update(users::table.filter(users::username.eq(name)))
            .set(users::password.eq(pass))
            .get_result(&mut conn)
    }

    fn create(&self, name: String, mail: String, pass: String) -> QueryResult<User> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");

        let new_user = User {
            id: Some(1),
            username: name,
            email: mail,
            password: pass,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&mut conn)
    }

    fn get_by_email(&self, mail: String) -> QueryResult<Option<User>> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        users::table
            .filter(users::email.eq(mail))
            .first::<User>(&mut conn)
            .optional()
    }
}
