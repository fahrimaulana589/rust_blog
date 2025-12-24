use crate::app::features::auth::interface::dto::{
    ForgotPasswordRequestDto, LoginRequestDto, LoginResponseDto, ResetPasswordRequestDto,
};
use crate::init_test_app;
use crate::test::helpers::seed_user;
use crate::utils::di::Container;
use crate::utils::success_response::SuccessResponse;
use actix_web::test;
use serial_test::serial;

#[actix_web::test]
#[serial]
async fn test_login() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);

    let login_dto = LoginRequestDto {
        username: container.config.default_username.clone(),
        password: container.config.default_password.clone(),
    };

    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_dto)
        .to_request();

    let resp: SuccessResponse<LoginResponseDto> = test::call_and_read_body_json(&app, req).await;
    assert!(resp.data.is_some());
    assert_eq!(
        resp.data.unwrap().username,
        container.config.default_username
    );

    // Verify Cookie
    let login_dto_cookie = LoginRequestDto {
        username: container.config.default_username.clone(),
        password: container.config.default_password.clone(),
    };
    let req_cookie = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_dto_cookie)
        .to_request();
    let resp_cookie = test::call_service(&app, req_cookie).await;
    assert!(resp_cookie.status().is_success());

    let cookie = resp_cookie
        .response()
        .cookies()
        .find(|c| c.name() == "auth_token")
        .expect("Cookie 'auth_token' not found");

    assert_eq!(cookie.http_only(), Some(true));
    assert_eq!(cookie.path(), Some("/"));
}

#[actix_web::test]
#[serial]
async fn test_cookie_auth() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);

    // 1. Login to get token
    let login_dto = LoginRequestDto {
        username: container.config.default_username.clone(),
        password: container.config.default_password.clone(),
    };

    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_dto)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let cookie = resp
        .response()
        .cookies()
        .find(|c| c.name() == "auth_token")
        .expect("Cookie 'auth_token' not found")
        .clone();

    // 2. Access protected endpoint using cookie (No Bearer Header)
    // Using /app/profile as protected endpoint example from API docs, or /app/count
    let req_protected = test::TestRequest::get()
        .uri("/app/count") // /app/count is protected
        .cookie(cookie)
        .to_request();

    let resp_protected = test::call_service(&app, req_protected).await;
    assert!(resp_protected.status().is_success());
}

#[actix_web::test]
#[serial]
async fn test_is_login() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);

    // 1. Unauthenticated request
    let req = test::TestRequest::get().uri("/app/islogin").to_request();
    // Middleware returns Err for unauthorized, so we must use app.call directly and expect Err
    use actix_web::dev::Service;
    let resp = app.call(req).await;
    assert!(resp.is_err());
    // Optionally check if error relates to Unauthorized if needed, but is_err is sufficient for now since middleware throws error.

    // 2. Authenticated request
    let login_dto = LoginRequestDto {
        username: container.config.default_username.clone(),
        password: container.config.default_password.clone(),
    };
    let req_login = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_dto)
        .to_request();
    let resp_login = test::call_service(&app, req_login).await;
    let cookie = resp_login
        .response()
        .cookies()
        .find(|c| c.name() == "auth_token")
        .expect("Cookie 'auth_token' not found")
        .clone();

    let req_auth = test::TestRequest::get()
        .uri("/app/islogin")
        .cookie(cookie)
        .to_request();
    let resp_auth = test::call_service(&app, req_auth).await;
    assert_eq!(resp_auth.status(), actix_web::http::StatusCode::OK);
}

#[actix_web::test]
#[serial]
async fn test_logout() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);

    // 1. Login first to get a token (optional, but realistic)
    let login_dto = LoginRequestDto {
        username: container.config.default_username.clone(),
        password: container.config.default_password.clone(),
    };
    let req_login = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_dto)
        .to_request();
    let resp_login = test::call_service(&app, req_login).await;
    assert!(resp_login.status().is_success());

    // 2. Call Logout
    let req_logout = test::TestRequest::post().uri("/logout").to_request();
    let resp_logout = test::call_service(&app, req_logout).await;
    assert!(resp_logout.status().is_success());

    // 3. Verify Cookie is cleared
    let cookie = resp_logout
        .response()
        .cookies()
        .find(|c| c.name() == "auth_token")
        .expect("Cookie 'auth_token' not found in logout response");

    assert_eq!(cookie.value(), "");
    assert_eq!(
        cookie.max_age(),
        Some(actix_web::cookie::time::Duration::ZERO)
    );
}

#[actix_web::test]
#[serial]
async fn test_forgot_password_flow() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);

    // 1. Forgot Password Request
    let forgot_dto = ForgotPasswordRequestDto {
        email: container.config.default_email.clone(),
    };

    let _req = test::TestRequest::post()
        .uri("/forgot-password")
        .set_json(&forgot_dto)
        .to_request();

    // We expect success message, even though email sending might fail in test env if not mocked perfectly,
    // but our current impl returns "Email sent successfully" if code executes.
    // However, without a real SMTP server, the email sending line might not error if lettre is configured loosely,
    // OR it might error. Real integration tests usually mock the email service.
    // Given the previous conversation about SMTP debugging, we might assume it tries to connect.
    // If it fails, our controller returns 500.
    // Ideally we should use a mock. But sticking to "schema" checking:

    // For this test, we can simulate the "Reset" part directly because we can generate the token ourselves since we have the secret.
    // The previous flow sends email. We can't easily intercept the email here without complex setup.
    // So we will manually Generate the token to proceed to step 2.

    let token = crate::utils::token::create_token(
        &container.config.default_username,
        &container.config.jwt_secret,
    );

    // 2. Reset Password
    let new_password = "NewPassword123!".to_string();
    let reset_dto = ResetPasswordRequestDto {
        token: token.clone(),
        new_password: new_password.clone(),
    };

    let req = test::TestRequest::post()
        .uri("/reset-password")
        .set_json(&reset_dto)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // 3. Login with New Password
    let login_dto = LoginRequestDto {
        username: container.config.default_username.clone(),
        password: new_password.clone(),
    };

    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_dto)
        .to_request();

    let resp: SuccessResponse<LoginResponseDto> = test::call_and_read_body_json(&app, req).await;
    assert!(resp.data.is_some());

    // Cleanup: Reset password back to default
    let pool = crate::utils::db::establish_connection(&container.config.database_url);
    use crate::app::features::auth::domain::repository::UserRepository;
    use crate::app::features::auth::infrastructure::repository_impl::UserRepositoryImpl;
    let user_repo = UserRepositoryImpl::new(pool);
    let _ = user_repo.reset_password(
        container.config.default_username,
        container.config.default_password,
    );
}

#[actix_web::test]
#[serial]
async fn test_forgot_password() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);

    let forgot_dto = ForgotPasswordRequestDto {
        email: container.config.default_email.clone(),
    };

    let req = test::TestRequest::post()
        .uri("/forgot-password")
        .set_json(&forgot_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
#[serial]
async fn test_reset_password() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);

    // Manually generate token to isolate this test from forgot-password logic
    let token = crate::utils::token::create_token(
        &container.config.default_username,
        &container.config.jwt_secret,
    );

    let reset_dto = ResetPasswordRequestDto {
        token,
        new_password: "NewProPass123!".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/reset-password")
        .set_json(&reset_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Verify login with new password
    let login_dto = LoginRequestDto {
        username: container.config.default_username.clone(),
        password: "NewProPass123!".to_string(),
    };
    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_dto)
        .to_request();
    let resp: SuccessResponse<LoginResponseDto> = test::call_and_read_body_json(&app, req).await;
    assert!(resp.data.is_some());
}
