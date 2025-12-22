use super::super::super::projects::domain::entity::{Project, Stack};
use super::super::domain::entity::{NewPortfolio, Portfolio};
use super::super::domain::repository::PortfolioRepository;
use crate::schema::{portfolios, project_stack, projects, stacks};
use crate::utils::db::DbPool;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::{QueryDsl, RunQueryDsl};

pub struct PortfolioRepositoryImpl {
    pub pool: DbPool,
}

impl PortfolioRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    fn get_stacks_for_project(
        &self,
        conn: &mut SqliteConnection,
        project_id: i32,
    ) -> QueryResult<Vec<Stack>> {
        project_stack::table
            .inner_join(stacks::table)
            .filter(project_stack::project_id.eq(project_id))
            .select(Stack::as_select())
            .load(conn)
    }
}

impl PortfolioRepository for PortfolioRepositoryImpl {
    fn create(&self, new_portfolio: NewPortfolio) -> QueryResult<(Portfolio, Project, Vec<Stack>)> {
        let mut conn = self.pool.get().expect("Failed to get db connection");

        let portfolio: Portfolio = diesel::insert_into(portfolios::table)
            .values(&new_portfolio)
            .returning(Portfolio::as_returning())
            .get_result(&mut conn)?;

        let project: Project = projects::table
            .find(portfolio.project_id)
            .select(Project::as_select())
            .get_result(&mut conn)?;

        let stacks = self.get_stacks_for_project(&mut conn, project.id)?;

        Ok((portfolio, project, stacks))
    }

    fn find_all(
        &self,
        offset: i64,
        limit: i64,
    ) -> QueryResult<(Vec<(Portfolio, Project, Vec<Stack>)>, i64)> {
        let mut conn = self.pool.get().expect("Failed to get db connection");

        let items: Vec<(Portfolio, Project)> = portfolios::table
            .inner_join(projects::table)
            .select((Portfolio::as_select(), Project::as_select()))
            .order(portfolios::created_at.desc())
            .offset(offset)
            .limit(limit)
            .load(&mut conn)?;

        let mut results = Vec::new();
        for (portfolio, project) in items {
            let stacks = self.get_stacks_for_project(&mut conn, project.id)?;
            results.push((portfolio, project, stacks));
        }

        let total_count: i64 = portfolios::table.count().get_result(&mut conn)?;

        Ok((results, total_count))
    }

    fn find_by_id(&self, id: i32) -> QueryResult<(Portfolio, Project, Vec<Stack>)> {
        let mut conn = self.pool.get().expect("Failed to get db connection");

        let (portfolio, project): (Portfolio, Project) = portfolios::table
            .find(id)
            .inner_join(projects::table)
            .select((Portfolio::as_select(), Project::as_select()))
            .get_result(&mut conn)?;

        let stacks = self.get_stacks_for_project(&mut conn, project.id)?;

        Ok((portfolio, project, stacks))
    }

    fn update(
        &self,
        id: i32,
        portfolio_data: NewPortfolio,
    ) -> QueryResult<(Portfolio, Project, Vec<Stack>)> {
        let mut conn = self.pool.get().expect("Failed to get db connection");

        let portfolio: Portfolio = diesel::update(portfolios::table.find(id))
            .set((
                portfolios::project_id.eq(portfolio_data.project_id),
                portfolios::judul.eq(portfolio_data.judul),
                portfolios::deskripsi.eq(portfolio_data.deskripsi),
                portfolios::is_active.eq(portfolio_data.is_active),
                portfolios::updated_at.eq(diesel::dsl::now),
            ))
            .returning(Portfolio::as_returning())
            .get_result(&mut conn)?;

        let project: Project = projects::table
            .find(portfolio.project_id)
            .select(Project::as_select())
            .get_result(&mut conn)?;

        let stacks = self.get_stacks_for_project(&mut conn, project.id)?;

        Ok((portfolio, project, stacks))
    }

    fn delete(&self, id: i32) -> QueryResult<usize> {
        let mut conn = self.pool.get().expect("Failed to get db connection");

        diesel::delete(portfolios::table.find(id)).execute(&mut conn)
    }
}
