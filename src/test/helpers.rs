use crate::app::features::auth::domain::repository::UserRepository;
use crate::app::features::auth::infrastructure::repository_impl::UserRepositoryImpl;
use crate::app::features::auth::interface::dto::{LoginRequestDto, UserResponseDto};
use crate::utils::db::establish_connection;
use crate::utils::di::Container;
use crate::utils::success_response::SuccessResponse;
use actix_http;
use actix_web::test;

#[macro_export]
macro_rules! init_test_app {
    ($container:expr) => {{
        use crate::app::drivers::routes;
        use actix_web::{App, test, web};
        test::init_service(
            App::new()
                .app_data(web::Data::new($container.clone()))
                .configure(routes::routes),
        )
        .await
    }};
}

pub fn seed_user(container: &Container) {
    let pool = establish_connection(&container.config.database_url);
    let user_repo = UserRepositoryImpl::new(pool);
    let username = container.config.default_username.clone();
    let email = container.config.default_email.clone();
    let password = container.config.default_password.clone();

    if let Err(_) = user_repo.create(username.clone(), email.clone(), password.clone()) {
        let _ = user_repo.reset_password(username, password);
    }
}

pub async fn login_admin(
    app: &impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
    container: &Container,
) -> String {
    let login_dto = LoginRequestDto {
        username: container.config.default_username.clone(),
        password: container.config.default_password.clone(),
    };

    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_dto)
        .to_request();

    let resp: SuccessResponse<UserResponseDto> = test::call_and_read_body_json(app, req).await;
    resp.data.expect("Login failed during test setup").token
}
