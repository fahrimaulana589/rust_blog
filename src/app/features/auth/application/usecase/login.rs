use crate::app::features::auth::domain::repository::UserRepository;
use std::sync::Arc;
use crate::app::features::auth::domain::entity::User;

#[derive(Clone)]
pub struct Execute {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl Execute {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repository }
    }

    pub fn execute(&self, username: String, password: String) -> Result<User, String> {
        let user = self.user_repository.get_where(username, password);
        match user {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err("User not found".to_string()),
            Err(e) => Err(e.to_string()),
        }
    }
}