use crate::schema::counts;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = counts)]
pub struct Counts {
    pub id: Option<i32>,
    pub count: Option<i32>,
}
