use super::super::super::projects::domain::entity::{Project, Stack};
use super::entity::{NewPortofolio, Portofolio};
use diesel::QueryResult;

pub trait PortofolioRepository: Send + Sync {
    fn create(
        &self,
        new_portfolio: NewPortofolio,
    ) -> QueryResult<(Portofolio, Project, Vec<Stack>)>;
    fn find_all(
        &self,
        offset: i64,
        limit: i64,
    ) -> QueryResult<(Vec<(Portofolio, Project, Vec<Stack>)>, i64)>;
    fn find_by_id(&self, id: i32) -> QueryResult<(Portofolio, Project, Vec<Stack>)>;
    fn update(
        &self,
        id: i32,
        portfolio_data: NewPortofolio,
    ) -> QueryResult<(Portofolio, Project, Vec<Stack>)>;
    fn delete(&self, id: i32) -> QueryResult<usize>;
    fn find_by_judul(&self, judul: String) -> QueryResult<Option<Portofolio>>;
}
