use super::super::super::projects::domain::entity::{Project, Stack};
use super::entity::{NewPortfolio, Portfolio};
use diesel::QueryResult;

pub trait PortfolioRepository: Send + Sync {
    fn create(&self, new_portfolio: NewPortfolio) -> QueryResult<(Portfolio, Project, Vec<Stack>)>;
    fn find_all(
        &self,
        offset: i64,
        limit: i64,
    ) -> QueryResult<(Vec<(Portfolio, Project, Vec<Stack>)>, i64)>;
    fn find_by_id(&self, id: i32) -> QueryResult<(Portfolio, Project, Vec<Stack>)>;
    fn update(
        &self,
        id: i32,
        portfolio_data: NewPortfolio,
    ) -> QueryResult<(Portfolio, Project, Vec<Stack>)>;
    fn delete(&self, id: i32) -> QueryResult<usize>;
    fn find_by_judul(&self, judul: String) -> QueryResult<Option<Portfolio>>;
}
