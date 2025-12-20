use crate::app::features::home::application::usecase::count::Execute;
use crate::app::features::home::domain::repository::CountRepository;
use crate::app::features::home::infrastructure::repository_impl::CountRepositoryImpl;
use crate::config::Config;
use crate::utils::db::establish_connection;
use std::sync::Arc;

#[derive(Clone)]
pub struct Container {
    pub config: Config,
    pub count_usecase: Execute,
}

impl Container {
    pub fn new() -> Self {
        let config = Config::new();
        let pool = establish_connection(&config.database_url);
        let count_repository: Arc<dyn CountRepository + Send + Sync> =
            Arc::new(CountRepositoryImpl::new(pool.clone()));
        let count_usecase = Execute::new(count_repository);
        Self {
            config,
            count_usecase,
        }
    }
}
