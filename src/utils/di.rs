use crate::app::features::auth::domain::repository::UserRepository;
use crate::app::features::auth::infrastructure::repository_impl::UserRepositoryImpl;
use crate::app::features::home::application::usecase as home_usecase;

use crate::app::features::auth::application::usecase as auth_usecase;
use crate::app::features::home::domain::repository::CountRepository;
use crate::app::features::home::infrastructure::repository_impl::CountRepositoryImpl;
use crate::config::Config;
use crate::utils::db::establish_connection;
use crate::utils::email::Email;
use std::sync::Arc;

#[derive(Clone)]
pub struct Container {
    pub config: Config,
    pub count_usecase: home_usecase::count::Execute,
    pub login_usecase: auth_usecase::login::Execute,
    pub send_email_usecase: home_usecase::send_email::Execute,
    pub forgot_password_usecase: auth_usecase::forgot_password::Execute,
}

impl Container {
    pub fn new() -> Self {
        let config = Config::new();

        let pool = establish_connection(&config.database_url);
        let email = Email::new(config.clone());

        let count_repository: Arc<dyn CountRepository + Send + Sync> =
            Arc::new(CountRepositoryImpl::new(pool.clone()));
        let count_usecase = home_usecase::count::Execute::new(count_repository);

        let user_repository: Arc<dyn UserRepository + Send + Sync> =
            Arc::new(UserRepositoryImpl::new(pool.clone()));
        let login_usecase =
            auth_usecase::login::Execute::new(user_repository.clone(), config.clone());

        let send_email_usecase = home_usecase::send_email::Execute::new(email.clone());
        let forgot_password_usecase =
            auth_usecase::forgot_password::Execute::new(user_repository.clone(), email.clone());

        Self {
            config,
            count_usecase,
            login_usecase,
            send_email_usecase,
            forgot_password_usecase,
        }
    }
}
