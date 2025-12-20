use crate::app::features::auth::domain::repository::UserRepository;
use crate::utils::email::Email;
use serde_json::json;
use std::sync::Arc;

#[derive(Clone)]
pub struct Execute {
    pub user_repository: Arc<dyn UserRepository + Send + Sync>,
    pub email: Email,
}

impl Execute {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>, email: Email) -> Self {
        Self { user_repository, email }
    }
}

impl Execute {
    pub fn execute(&self, email: String) -> Result<String, String> {
        let user = self.user_repository.get_by_email(email);
        match user {
            Ok(user) => {
                if let Some(user_data) = user {
                    let _result = self.email.send_email_to_user(
                        user_data.clone(), 
                        "Forgot Password".to_string(), 
                        json!({
                            "email": user_data.email,
                            "token": crate::utils::token::create_token(&user_data.username, &self.email.config.jwt_secret),
                            "message": "Please click the link below to reset your password"
                        }).to_string()
                    );
                    match _result {
                        Ok(_) => Ok("Email sent successfully".to_string()),
                        Err(e) => Err(e.to_string()),
                    }
                } else {
                     Err("User not found".to_string())
                }
            },
            Err(e) => Err(e.to_string()),
        }
    }
}