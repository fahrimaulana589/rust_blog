use super::super::super::projects::domain::entity::{Project, Stack};
use super::super::domain::entity::{NewPortofolio, Portofolio};
use super::super::domain::repository::PortofolioRepository;
use crate::schema::{portofolios, project_stack, projects, stacks};
use crate::utils::db::DbPool;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::{QueryDsl, RunQueryDsl};

pub struct PortofolioRepositoryImpl {
    pub pool: DbPool,
}

impl PortofolioRepositoryImpl {
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

impl PortofolioRepository for PortofolioRepositoryImpl {
    fn create(
        &self,
        new_portfolio: NewPortofolio,
    ) -> QueryResult<(Portofolio, Project, Vec<Stack>)> {
        let mut conn = self.pool.get().expect("Failed to get db connection");

        let portfolio: Portofolio = diesel::insert_into(portofolios::table)
            .values(&new_portfolio)
            .returning(Portofolio::as_returning())
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
    ) -> QueryResult<(Vec<(Portofolio, Project, Vec<Stack>)>, i64)> {
        let mut conn = self.pool.get().expect("Failed to get db connection");

        let items: Vec<(Portofolio, Project)> = portofolios::table
            .inner_join(projects::table)
            .select((Portofolio::as_select(), Project::as_select()))
            .order(portofolios::created_at.desc())
            .offset(offset)
            .limit(limit)
            .load(&mut conn)?;

        let mut results = Vec::new();
        for (portfolio, project) in items {
            let stacks = self.get_stacks_for_project(&mut conn, project.id)?;
            results.push((portfolio, project, stacks));
        }

        let total_count: i64 = portofolios::table.count().get_result(&mut conn)?;

        Ok((results, total_count))
    }

    fn find_by_id(&self, id: i32) -> QueryResult<(Portofolio, Project, Vec<Stack>)> {
        let mut conn = self.pool.get().expect("Failed to get db connection");

        let (portfolio, project): (Portofolio, Project) = portofolios::table
            .find(id)
            .inner_join(projects::table)
            .select((Portofolio::as_select(), Project::as_select()))
            .get_result(&mut conn)?;

        let stacks = self.get_stacks_for_project(&mut conn, project.id)?;

        Ok((portfolio, project, stacks))
    }

    fn update(
        &self,
        id: i32,
        portfolio_data: NewPortofolio,
    ) -> QueryResult<(Portofolio, Project, Vec<Stack>)> {
        let mut conn = self.pool.get().expect("Failed to get db connection");

        let portfolio: Portofolio = diesel::update(portofolios::table.find(id))
            .set((
                portofolios::project_id.eq(portfolio_data.project_id),
                portofolios::judul.eq(portfolio_data.judul),
                portofolios::deskripsi.eq(portfolio_data.deskripsi),
                portofolios::is_active.eq(portfolio_data.is_active),
                portofolios::slug.eq(portfolio_data.slug),
                portofolios::updated_at.eq(diesel::dsl::now),
            ))
            .returning(Portofolio::as_returning())
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

        diesel::delete(portofolios::table.find(id)).execute(&mut conn)
    }

    fn find_by_slug(&self, slug: String) -> QueryResult<Option<Portofolio>> {
        let mut conn = self.pool.get().expect("Failed to get db connection");

        portofolios::table
            .filter(portofolios::slug.eq(slug))
            .first(&mut conn)
            .optional()
    }
}
