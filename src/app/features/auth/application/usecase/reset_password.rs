use crate::app::features::auth::domain::repository::UserRepository;
use crate::config::Config;
use crate::utils::token::verify_token;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    pub user_repository: Arc<dyn UserRepository + Send + Sync>,
    pub config: Config,
}

impl Execute {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>, config: Config) -> Self {
        Self {
            user_repository,
            config,
        }
    }
}

impl Execute {
    pub fn execute(&self, token: String, new_password: String) -> Result<String, String> {
        let claims = verify_token(&token, &self.config.jwt_secret);
        match claims {
            Ok(token_data) => {
                let username = token_data.sub; // verify_token returns Claims directly
                match self.user_repository.reset_password(username, new_password) {
                    Ok(_) => Ok("Password reset successfully".to_string()),
                    Err(e) => Err(e.to_string()),
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
