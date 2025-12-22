use crate::app::features::auth::domain::repository::UserRepository;
use crate::app::features::auth::interface::dto::LoginResponseDto;
use crate::config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    config: Config,
}

impl Execute {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>, config: Config) -> Self {
        Self {
            user_repository,
            config,
        }
    }

    pub fn execute(&self, username: String, password: String) -> Result<LoginResponseDto, String> {
        // Check if user with ID 1 exists
        if let Ok(None) = self.user_repository.get(&1) {
            // Default user not found, create it
            let _ = self.user_repository.create(
                self.config.default_username.clone(),
                self.config.default_email.clone(),
                self.config.default_password.clone(),
            );
        }

        let user = self.user_repository.get_where(username, password);
        match user {
            Ok(Some(user)) => {
                let token =
                    crate::utils::token::create_token(&user.username, &self.config.jwt_secret);
                Ok(LoginResponseDto::from(user, token))
            }
            Ok(None) => Err("User not found".to_string()),
            Err(e) => Err(e.to_string()),
        }
    }
}
