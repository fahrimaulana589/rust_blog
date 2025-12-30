use crate::init_test_app;
use crate::utils::di::Container;
use actix_web::test;
use serial_test::serial;

#[actix_web::test]
#[serial]
async fn test_public_access_get_blogs() {
    let container = Container::new();
    let app = init_test_app!(&container);

    // No Authorization header
    let req = test::TestRequest::get()
        .uri("/app/blogs?page=1&per_page=10")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(
        resp.status().is_success(),
        "GET /app/blogs should be public"
    );
}

#[actix_web::test]
#[serial]
async fn test_public_access_get_projects() {
    let container = Container::new();
    let app = init_test_app!(&container);

    let req = test::TestRequest::get().uri("/app/projects").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(
        resp.status().is_success(),
        "GET /app/projects should be public"
    );
}

#[actix_web::test]
#[serial]
async fn test_public_access_get_portfolios() {
    let container = Container::new();
    let app = init_test_app!(&container);

    let req = test::TestRequest::get()
        .uri("/app/portofolios")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(
        resp.status().is_success(),
        "GET /app/portofolios should be public"
    );
}

#[actix_web::test]
#[serial]
async fn test_public_access_get_profile() {
    let container = Container::new();
    let app = init_test_app!(&container);

    // Profile might return null data but status should be 200 OK
    let req = test::TestRequest::get().uri("/app/profile").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(
        resp.status().is_success(),
        "GET /app/profile should be public"
    );
}
