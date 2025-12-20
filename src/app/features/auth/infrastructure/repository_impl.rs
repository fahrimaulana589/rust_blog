use crate::app::features::auth::domain::entity::User;
use crate::app::features::auth::domain::repository::UserRepository;
use crate::schema::users::dsl::*;
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
        let result = users
            .filter(crate::schema::users::id.eq(get_id))
            .first::<User>(&mut conn);
        match result {
            Ok(c) => Ok(Some(c)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }
    fn get_where(&self, name: String, pass: String) -> QueryResult<Option<User>> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        let result = users
            .filter(crate::schema::users::username.eq(name))
            .filter(crate::schema::users::password.eq(pass))
            .first::<User>(&mut conn);
        match result {
            Ok(c) => Ok(Some(c)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }
    fn _reset_password(&self, name: String, pass: String) -> QueryResult<User> {
        // Note: This logic currently behaves like get_where but returns Result<User> instead of Option.
        // It does not actually UPDATE the password. User requested "fix error", not "implement logic".
        // I will keep logic similar to what user wrote but compile-safe.
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        let result = users
            .filter(crate::schema::users::username.eq(name))
            .filter(crate::schema::users::password.eq(pass))
            .first::<User>(&mut conn);
        match result {
            Ok(c) => Ok(c),
            Err(e) => Err(e),
        }
    }

    fn create(&self, name: String, mail: String, pass: String) -> QueryResult<User> {
        use crate::app::features::auth::domain::entity::User;
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

        diesel::insert_into(users)
            .values(&new_user)
            .get_result(&mut conn)
    }

    fn get_by_email(&self, mail: String) -> QueryResult<Option<User>> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        let result = users
            .filter(crate::schema::users::email.eq(mail))
            .first::<User>(&mut conn);
        match result {
            Ok(c) => Ok(Some(c)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
