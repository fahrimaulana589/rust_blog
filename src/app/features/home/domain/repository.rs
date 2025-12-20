use crate::app::features::home::domain::entity::Counts;
use diesel::QueryResult;

pub trait CountRepository {
    fn get(&self) -> QueryResult<Option<Counts>>;
    fn increment(&self) -> QueryResult<i32>;
}
