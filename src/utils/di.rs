use crate::app::features::auth::domain::repository::UserRepository;
use crate::app::features::auth::infrastructure::repository_impl::UserRepositoryImpl;
use crate::app::features::home::application::usecase as home_usecase;

use crate::app::features::auth::application::usecase as auth_usecase;
use crate::app::features::blog::application::category_usecase;
use crate::app::features::blog::domain::repository::BlogRepository;
use crate::app::features::blog::infrastructure::repository_impl::BlogRepositoryImpl;
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
    pub reset_password_usecase: auth_usecase::reset_password::Execute,
    pub create_category_usecase: category_usecase::create::Execute,
    pub get_categories_usecase: category_usecase::get_all::Execute,
    pub get_category_usecase: category_usecase::get::Execute,
    pub update_category_usecase: category_usecase::update::Execute,
    pub delete_category_usecase: category_usecase::delete::Execute,
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
        let reset_password_usecase =
            auth_usecase::reset_password::Execute::new(user_repository.clone(), config.clone());

        let blog_repository: Arc<dyn BlogRepository + Send + Sync> =
            Arc::new(BlogRepositoryImpl::new(pool.clone()));

        let create_category_usecase =
            category_usecase::create::Execute::new(blog_repository.clone());
        let get_categories_usecase =
            category_usecase::get_all::Execute::new(blog_repository.clone());
        let get_category_usecase = category_usecase::get::Execute::new(blog_repository.clone());
        let update_category_usecase =
            category_usecase::update::Execute::new(blog_repository.clone());
        let delete_category_usecase =
            category_usecase::delete::Execute::new(blog_repository.clone());

        Self {
            config,
            count_usecase,
            login_usecase,
            send_email_usecase,
            forgot_password_usecase,
            reset_password_usecase,
            create_category_usecase,
            get_categories_usecase,
            get_category_usecase,
            update_category_usecase,
            delete_category_usecase,
        }
    }
}
