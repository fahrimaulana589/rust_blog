use crate::app::features::home::models::Counts;
use crate::schema::counts::dsl::*;
use crate::utils::db::DbPool;
use diesel::prelude::*;

#[derive(Clone)]
pub struct CountRepository {
    pool: DbPool,
}

impl CountRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn get(&self) -> QueryResult<Option<Counts>> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        let result = counts
            .filter(crate::schema::counts::id.eq(1))
            .first::<Counts>(&mut conn);
        match result {
            Ok(c) => Ok(Some(c)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn increment(&self) -> QueryResult<i32> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");

        let current_count = self.get()?;

        if current_count.is_none() {
            diesel::insert_into(counts)
                .values((
                    crate::schema::counts::id.eq(1),
                    crate::schema::counts::count.eq(0),
                ))
                .execute(&mut conn)?;
        }

        diesel::update(counts.filter(crate::schema::counts::id.eq(1)))
            .set(crate::schema::counts::count.eq(crate::schema::counts::count + 1))
            .execute(&mut conn)?;

        let new_count_struct = counts
            .filter(crate::schema::counts::id.eq(1))
            .first::<Counts>(&mut conn)?;

        Ok(new_count_struct.count.unwrap_or(0))
    }
}
