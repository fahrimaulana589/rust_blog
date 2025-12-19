use crate::app::features::home::repository::CountRepository;
use crate::config::Config;
use crate::utils::db::establish_connection;

#[derive(Clone)]
pub struct Container {
    pub config: Config,
    pub count_repository: CountRepository,
}

impl Container {
    pub fn new() -> Self {
        let config = Config::new();
        let pool = establish_connection(&config.database_url);
        let count_repository = CountRepository::new(pool.clone());
        Self {
            config,
            count_repository,
        }
    }
}
