use crate::app::features::auth::domain::repository::UserRepository;
use crate::app::features::auth::infrastructure::repository_impl::UserRepositoryImpl;
use crate::app::features::home::application::usecase as home_usecase;

use crate::app::features::home::domain::repository::CountRepository;
use crate::app::features::home::infrastructure::repository_impl::CountRepositoryImpl;
use crate::app::features::auth::application::usecase as auth_usecase;
use crate::config::Config;
use crate::utils::db::establish_connection;
use std::sync::Arc;

#[derive(Clone)]
pub struct Container {
    pub config: Config,
    pub count_usecase: home_usecase::count::Execute,
    pub login_usecase: auth_usecase::login::Execute,
}

impl Container {
    pub fn new() -> Self {
        let config = Config::new();

        let pool = establish_connection(&config.database_url);

        let count_repository: Arc<dyn CountRepository + Send + Sync> =
            Arc::new(CountRepositoryImpl::new(pool.clone()));
        let count_usecase = home_usecase::count::Execute::new(count_repository);

        let user_repository: Arc<dyn UserRepository + Send + Sync> =
            Arc::new(UserRepositoryImpl::new(pool.clone()));
        let login_usecase = auth_usecase::login::Execute::new(user_repository);

        Self {
            config,
            count_usecase,
            login_usecase,
        }
    }
}
