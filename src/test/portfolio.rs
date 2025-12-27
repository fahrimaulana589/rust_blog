use crate::app::features::portfolio::interface::dto::{
    CreatePortfolioRequestDto, PortfolioResponseDto, UpdatePortfolioRequestDto,
};
use crate::app::features::projects::interface::dto::{CreateProjectRequestDto, ProjectResponseDto};
use crate::init_test_app;
use crate::test::helpers::{login_admin, seed_user};
use crate::utils::di::Container;
use crate::utils::success_response::SuccessResponse;
use actix_web::test;
use chrono::Utc;
use serial_test::serial;

#[actix_web::test]
#[serial]
async fn test_create_portfolio() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // 1. Create Project
    let project_name = format!("Project P1 {}", Utc::now().timestamp_micros());
    let create_project_dto = CreateProjectRequestDto {
        nama_projek: project_name,
        deskripsi: "Desc".to_string(),
        status: "ongoing".to_string(),
        progress: 0,
        link_demo: None,
        repository: None,
        tanggal_mulai: "2023-01-01".to_string(),
        tanggal_selesai: None,
        stack_ids: None,
    };
    let req = test::TestRequest::post()
        .uri("/app/projects")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_project_dto)
        .to_request();
    let resp: SuccessResponse<ProjectResponseDto> = test::call_and_read_body_json(&app, req).await;
    let project_id = resp.data.unwrap().id;

    // 2. Create Portfolio
    let portfolio_title = format!("Portfolio 1 {}", Utc::now().timestamp_micros());
    let create_portfolio_dto = CreatePortfolioRequestDto {
        project_id,
        judul: portfolio_title.clone(),
        deskripsi: "Portfolio Desc".to_string(),
        is_active: true,
    };

    let req = test::TestRequest::post()
        .uri("/app/portfolios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_portfolio_dto)
        .to_request();
    let resp: SuccessResponse<PortfolioResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let portfolio = resp.data.unwrap();

    assert_eq!(portfolio.judul, portfolio_title);
    assert_eq!(portfolio.project.id, project_id);
    assert_eq!(portfolio.deskripsi, Some("Portfolio Desc".to_string()));
    assert_eq!(portfolio.is_active, true);
}

#[actix_web::test]
#[serial]
async fn test_get_portfolios() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // 1. Create Project
    let project_name = format!("Project P2 {}", Utc::now().timestamp_micros());
    let create_project_dto = CreateProjectRequestDto {
        nama_projek: project_name,
        deskripsi: "Desc".to_string(),
        status: "ongoing".to_string(),
        progress: 0,
        link_demo: None,
        repository: None,
        tanggal_mulai: "2023-01-01".to_string(),
        tanggal_selesai: None,
        stack_ids: None,
    };
    let req = test::TestRequest::post()
        .uri("/app/projects")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_project_dto)
        .to_request();
    let resp: SuccessResponse<ProjectResponseDto> = test::call_and_read_body_json(&app, req).await;
    let project_id = resp.data.unwrap().id;

    // 2. Create Portfolio
    let portfolio_title = format!("Portfolio 2 {}", Utc::now().timestamp_micros());
    let create_portfolio_dto = CreatePortfolioRequestDto {
        project_id,
        judul: portfolio_title.clone(),
        deskripsi: "Desc".to_string(),
        is_active: true,
    };
    let req = test::TestRequest::post()
        .uri("/app/portfolios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_portfolio_dto)
        .to_request();
    test::call_service(&app, req).await;

    // 3. Get All
    let req = test::TestRequest::get()
        .uri("/app/portfolios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<
        crate::app::features::portfolio::interface::dto::PaginatedResponseDto<PortfolioResponseDto>,
    > = test::call_and_read_body_json(&app, req).await;

    let items = resp.data.unwrap().items;
    assert!(items.len() > 0);
    assert!(items.iter().any(|i| i.judul == portfolio_title));
}

#[actix_web::test]
#[serial]
async fn test_get_portfolio_by_id() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // 1. Create Project
    let project_name = format!("Project P3 {}", Utc::now().timestamp_micros());
    let create_project_dto = CreateProjectRequestDto {
        nama_projek: project_name,
        deskripsi: "Desc".to_string(),
        status: "ongoing".to_string(),
        progress: 0,
        link_demo: None,
        repository: None,
        tanggal_mulai: "2023-01-01".to_string(),
        tanggal_selesai: None,
        stack_ids: None,
    };
    let req = test::TestRequest::post()
        .uri("/app/projects")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_project_dto)
        .to_request();
    let resp: SuccessResponse<ProjectResponseDto> = test::call_and_read_body_json(&app, req).await;
    let project_id = resp.data.unwrap().id;

    // 2. Create Portfolio
    let portfolio_title = format!("Portfolio 3 {}", Utc::now().timestamp_micros());
    let create_portfolio_dto = CreatePortfolioRequestDto {
        project_id,
        judul: portfolio_title.clone(),
        deskripsi: "Desc".to_string(),
        is_active: true,
    };
    let req = test::TestRequest::post()
        .uri("/app/portfolios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_portfolio_dto)
        .to_request();
    let resp: SuccessResponse<PortfolioResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let portfolio_id = resp.data.unwrap().id;

    // 3. Get By ID
    let req = test::TestRequest::get()
        .uri(&format!("/app/portfolios/{}", portfolio_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<PortfolioResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let portfolio = resp.data.unwrap();

    assert_eq!(portfolio.judul, portfolio_title);
    assert_eq!(portfolio.is_active, true); // Default
}

#[actix_web::test]
#[serial]
async fn test_update_portfolio() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // 1. Create Project
    let project_name = format!("Project P4 {}", Utc::now().timestamp_micros());
    let create_project_dto = CreateProjectRequestDto {
        nama_projek: project_name,
        deskripsi: "Desc".to_string(),
        status: "ongoing".to_string(),
        progress: 0,
        link_demo: None,
        repository: None,
        tanggal_mulai: "2023-01-01".to_string(),
        tanggal_selesai: None,
        stack_ids: None,
    };
    let req = test::TestRequest::post()
        .uri("/app/projects")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_project_dto)
        .to_request();
    let resp: SuccessResponse<ProjectResponseDto> = test::call_and_read_body_json(&app, req).await;
    let project_id = resp.data.unwrap().id;

    // 2. Create Portfolio
    let portfolio_title = format!("Portfolio 4 {}", Utc::now().timestamp_micros());
    let create_portfolio_dto = CreatePortfolioRequestDto {
        project_id,
        judul: portfolio_title.clone(),
        deskripsi: "Desc".to_string(),
        is_active: true,
    };
    let req = test::TestRequest::post()
        .uri("/app/portfolios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_portfolio_dto)
        .to_request();
    let resp: SuccessResponse<PortfolioResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let portfolio_id = resp.data.unwrap().id;

    // 3. Update
    let update_title = format!("Portfolio Updated {}", Utc::now().timestamp_micros());
    let update_dto = UpdatePortfolioRequestDto {
        project_id: project_id, // Keep existing implicitly by sending same ID
        judul: update_title.clone(),
        deskripsi: "Updated Desc".to_string(),
        is_active: false,
    };
    let req = test::TestRequest::put()
        .uri(&format!("/app/portfolios/{}", portfolio_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&update_dto)
        .to_request();
    let resp: SuccessResponse<PortfolioResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let portfolio = resp.data.unwrap();

    assert_eq!(portfolio.judul, update_title);
    assert_eq!(portfolio.deskripsi, Some("Updated Desc".to_string()));
    assert_eq!(portfolio.is_active, false);

    // 4. Verify (Optional double check via Get, but Update response is trusted now)
    let req = test::TestRequest::get()
        .uri(&format!("/app/portfolios/{}", portfolio_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<PortfolioResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let portfolio_get = resp.data.unwrap();

    assert_eq!(portfolio_get.judul, update_title);
    assert_eq!(portfolio_get.is_active, false);
}

#[actix_web::test]
#[serial]
async fn test_delete_portfolio() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // 1. Create Project
    let project_name = format!("Project P5 {}", Utc::now().timestamp_micros());
    let create_project_dto = CreateProjectRequestDto {
        nama_projek: project_name,
        deskripsi: "Desc".to_string(),
        status: "ongoing".to_string(),
        progress: 0,
        link_demo: None,
        repository: None,
        tanggal_mulai: "2023-01-01".to_string(),
        tanggal_selesai: None,
        stack_ids: None,
    };
    let req = test::TestRequest::post()
        .uri("/app/projects")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_project_dto)
        .to_request();
    let resp: SuccessResponse<ProjectResponseDto> = test::call_and_read_body_json(&app, req).await;
    let project_id = resp.data.unwrap().id;

    // 2. Create Portfolio
    let portfolio_title = format!("Portfolio 5 {}", Utc::now().timestamp_micros());
    let create_portfolio_dto = CreatePortfolioRequestDto {
        project_id,
        judul: portfolio_title.clone(),
        deskripsi: "Desc".to_string(),
        is_active: true,
    };
    let req = test::TestRequest::post()
        .uri("/app/portfolios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_portfolio_dto)
        .to_request();
    let resp: SuccessResponse<PortfolioResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let portfolio_id = resp.data.unwrap().id;

    // 3. Delete
    let req = test::TestRequest::delete()
        .uri(&format!("/app/portfolios/{}", portfolio_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // 4. Verify Not Found
    let req = test::TestRequest::get()
        .uri(&format!("/app/portfolios/{}", portfolio_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
}
#[actix_web::test]
#[serial]
async fn test_create_duplicate_portfolio() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // 1. Create Project
    let project_name = format!("Project Dup {}", Utc::now().timestamp_micros());
    let create_project_dto = CreateProjectRequestDto {
        nama_projek: project_name,
        deskripsi: "Desc".to_string(),
        status: "ongoing".to_string(),
        progress: 0,
        link_demo: None,
        repository: None,
        tanggal_mulai: "2023-01-01".to_string(),
        tanggal_selesai: None,
        stack_ids: None,
    };
    let req = test::TestRequest::post()
        .uri("/app/projects")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_project_dto)
        .to_request();
    let resp: SuccessResponse<ProjectResponseDto> = test::call_and_read_body_json(&app, req).await;
    let project_id = resp.data.unwrap().id;

    // 2. Create Portfolio 1
    let portfolio_title = format!("Portfolio Dup {}", Utc::now().timestamp_micros());
    let create_portfolio_dto = CreatePortfolioRequestDto {
        project_id,
        judul: portfolio_title.clone(),
        deskripsi: "Portfolio Desc".to_string(),
        is_active: true,
    };

    let req = test::TestRequest::post()
        .uri("/app/portfolios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_portfolio_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // 3. Create Duplicate Portfolio
    let req = test::TestRequest::post()
        .uri("/app/portfolios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_portfolio_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);

    use crate::utils::error_response::ErrorResponse;
    let body: ErrorResponse = test::read_body_json(resp).await;
    assert!(body.errors.is_some());
    assert_eq!(
        body.errors.unwrap().get("judul").unwrap(),
        common_validation_message()
    );
}

fn common_validation_message() -> &'static str {
    "Portfolio title already exists"
}
