use crate::app::features::projects::domain::entity::{
    NewProject, NewProjectStack, NewStack, Project, Stack,
};
use crate::app::features::projects::domain::repository::ProjectRepository;
use crate::schema::{project_stack, projects, stacks};
use crate::utils::db::DbPool;
use diesel::QueryResult;
use diesel::prelude::*;

pub struct ProjectRepositoryImpl {
    pool: DbPool,
}

impl ProjectRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl ProjectRepository for ProjectRepositoryImpl {
    // --- Project ---
    fn get_all_projects(&self, limit: i64, offset: i64) -> QueryResult<(Vec<Project>, i64)> {
        let mut conn = self.pool.get().unwrap();

        let items = projects::table
            .limit(limit)
            .offset(offset)
            .order(projects::created_at.desc())
            .load::<Project>(&mut conn)?;

        let total_count: i64 = projects::table.count().get_result(&mut conn)?;

        Ok((items, total_count))
    }

    fn get_project_by_id(&self, id: i32) -> QueryResult<Option<Project>> {
        let mut conn = self.pool.get().unwrap();
        projects::table
            .find(id)
            .first::<Project>(&mut conn)
            .optional()
    }

    fn create_project(&self, project: NewProject) -> QueryResult<Project> {
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(projects::table)
            .values(&project)
            .get_result(&mut conn)
    }

    fn update_project(&self, id: i32, project: NewProject) -> QueryResult<Project> {
        let mut conn = self.pool.get().unwrap();
        diesel::update(projects::table.find(id))
            .set((
                projects::nama_projek.eq(project.nama_projek),
                projects::deskripsi.eq(project.deskripsi),
                projects::status.eq(project.status),
                projects::progress.eq(project.progress),
                projects::link_demo.eq(project.link_demo),
                projects::repository.eq(project.repository),
                projects::tanggal_mulai.eq(project.tanggal_mulai),
                projects::tanggal_selesai.eq(project.tanggal_selesai),
                projects::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .get_result(&mut conn)
    }

    fn delete_project(&self, id: i32) -> QueryResult<usize> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(projects::table.find(id)).execute(&mut conn)
    }

    fn get_project_by_name(&self, name: &str) -> QueryResult<Option<Project>> {
        let mut conn = self.pool.get().unwrap();
        projects::table
            .filter(projects::nama_projek.eq(name))
            .first::<Project>(&mut conn)
            .optional()
    }

    // --- Stack ---
    fn get_all_stacks(&self, limit: i64, offset: i64) -> QueryResult<(Vec<Stack>, i64)> {
        let mut conn = self.pool.get().unwrap();

        let items = stacks::table
            .limit(limit)
            .offset(offset)
            .order(stacks::created_at.desc())
            .load::<Stack>(&mut conn)?;

        let total_count: i64 = stacks::table.count().get_result(&mut conn)?;

        Ok((items, total_count))
    }

    fn get_stack_by_id(&self, id: i32) -> QueryResult<Option<Stack>> {
        let mut conn = self.pool.get().unwrap();
        stacks::table.find(id).first::<Stack>(&mut conn).optional()
    }

    fn create_stack(&self, stack: NewStack) -> QueryResult<Stack> {
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(stacks::table)
            .values(&stack)
            .get_result(&mut conn)
    }

    fn get_stack_by_name(&self, name: &str) -> QueryResult<Option<Stack>> {
        let mut conn = self.pool.get().unwrap();
        stacks::table
            .filter(stacks::nama_stack.eq(name))
            .first::<Stack>(&mut conn)
            .optional()
    }

    fn update_stack(&self, id: i32, stack: NewStack) -> QueryResult<Stack> {
        let mut conn = self.pool.get().unwrap();
        diesel::update(stacks::table.find(id))
            .set((
                stacks::nama_stack.eq(stack.nama_stack),
                stacks::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .get_result(&mut conn)
    }

    fn delete_stack(&self, id: i32) -> QueryResult<usize> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(stacks::table.find(id)).execute(&mut conn)
    }

    // --- Relations ---
    fn add_stack_to_project(&self, project_id: i32, stack_id: i32) -> QueryResult<()> {
        let mut conn = self.pool.get().unwrap();
        let new_relation = NewProjectStack {
            project_id,
            stack_id,
        };
        diesel::insert_into(project_stack::table)
            .values(&new_relation)
            .execute(&mut conn)?;
        Ok(())
    }

    fn remove_all_stacks_from_project(&self, project_id: i32) -> QueryResult<usize> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(project_stack::table.filter(project_stack::project_id.eq(project_id)))
            .execute(&mut conn)
    }

    fn get_stacks_by_project_id(&self, project_id: i32) -> QueryResult<Vec<Stack>> {
        let mut conn = self.pool.get().unwrap();
        project_stack::table
            .inner_join(stacks::table)
            .filter(project_stack::project_id.eq(project_id))
            .select(stacks::all_columns)
            .load::<Stack>(&mut conn)
    }
}
