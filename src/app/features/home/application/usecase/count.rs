use crate::app::features::home::domain::repository::CountRepository;
use diesel::QueryResult;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    pub count_repository: Arc<dyn CountRepository + Send + Sync>,
}

impl Execute {
    pub fn new(count_repository: Arc<dyn CountRepository + Send + Sync>) -> Self {
        Self { count_repository }
    }

    pub fn increment(&self) -> QueryResult<i32> {
        self.count_repository.increment()
    }
}
