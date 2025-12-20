use crate::schema::users;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub email: String,
    pub password: String,
}