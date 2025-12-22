use super::entity::{NewProject, NewStack, Project, Stack};
use diesel::QueryResult;

pub trait ProjectRepository {
    // Project CRUD
    fn get_all_projects(&self, limit: i64, offset: i64) -> QueryResult<(Vec<Project>, i64)>;
    fn get_project_by_id(&self, id: i32) -> QueryResult<Option<Project>>;
    fn create_project(&self, project: NewProject) -> QueryResult<Project>;
    fn update_project(&self, id: i32, project: NewProject) -> QueryResult<Project>;
    fn delete_project(&self, id: i32) -> QueryResult<usize>;

    // Stack CRUD
    fn get_all_stacks(&self) -> QueryResult<Vec<Stack>>;
    fn get_stack_by_id(&self, id: i32) -> QueryResult<Option<Stack>>;
    fn create_stack(&self, stack: NewStack) -> QueryResult<Stack>;
    fn get_stack_by_name(&self, name: &str) -> QueryResult<Option<Stack>>;
    fn update_stack(&self, id: i32, stack: NewStack) -> QueryResult<Stack>;
    fn delete_stack(&self, id: i32) -> QueryResult<usize>;

    // Relations
    fn add_stack_to_project(&self, project_id: i32, stack_id: i32) -> QueryResult<()>;
    fn remove_all_stacks_from_project(&self, project_id: i32) -> QueryResult<usize>;
    fn get_stacks_by_project_id(&self, project_id: i32) -> QueryResult<Vec<Stack>>;
}
