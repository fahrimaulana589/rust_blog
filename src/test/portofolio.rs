use crate::app::features::portofolio::interface::dto::{
    CreatePortofolioRequestDto, PortofolioResponseDto, UpdatePortofolioRequestDto,
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
async fn test_create_portofolio() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // 1. Create Project
    let project_name = format!("Project P1 {}", Utc::now().timestamp_micros());
    let create_project_dto = CreateProjectRequestDto {
        nama_projek: project_name,
        deskripsi: "Desc".to_string(),
        status: "ONGOING".to_string(),
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

    // 2. Create Portofolio
    let portofolio_title = format!("Portofolio 1 {}", Utc::now().timestamp_micros());
    let create_portofolio_dto = CreatePortofolioRequestDto {
        project_id,
        judul: portofolio_title.clone(),
        deskripsi: "Portofolio Desc".to_string(),
        is_active: true,
    };

    let req = test::TestRequest::post()
        .uri("/app/portofolios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_portofolio_dto)
        .to_request();
    let resp: SuccessResponse<PortofolioResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let portofolio = resp.data.unwrap();

    assert_eq!(portofolio.judul, portofolio_title);
    assert_eq!(portofolio.project.id, project_id);
    assert_eq!(portofolio.deskripsi, Some("Portofolio Desc".to_string()));
    assert_eq!(portofolio.is_active, true);
}

#[actix_web::test]
#[serial]
async fn test_get_portofolios() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // 1. Create Project
    let project_name = format!("Project P2 {}", Utc::now().timestamp_micros());
    let create_project_dto = CreateProjectRequestDto {
        nama_projek: project_name,
        deskripsi: "Desc".to_string(),
        status: "ONGOING".to_string(),
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

    // 2. Create Portofolio
    let portofolio_title = format!("Portofolio 2 {}", Utc::now().timestamp_micros());
    let create_portofolio_dto = CreatePortofolioRequestDto {
        project_id,
        judul: portofolio_title.clone(),
        deskripsi: "Desc".to_string(),
        is_active: true,
    };
    let req = test::TestRequest::post()
        .uri("/app/portofolios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_portofolio_dto)
        .to_request();
    test::call_service(&app, req).await;

    // 3. Get All
    let req = test::TestRequest::get()
        .uri("/app/portofolios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<
        crate::app::features::portofolio::interface::dto::PaginatedResponseDto<
            PortofolioResponseDto,
        >,
    > = test::call_and_read_body_json(&app, req).await;

    let items = resp.data.unwrap().items;
    assert!(items.len() > 0);
    assert!(items.iter().any(|i| i.judul == portofolio_title));
}

#[actix_web::test]
#[serial]
async fn test_get_portofolio_by_id() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // 1. Create Project
    let project_name = format!("Project P3 {}", Utc::now().timestamp_micros());
    let create_project_dto = CreateProjectRequestDto {
        nama_projek: project_name,
        deskripsi: "Desc".to_string(),
        status: "ONGOING".to_string(),
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

    // 2. Create Portofolio
    let portofolio_title = format!("Portofolio 3 {}", Utc::now().timestamp_micros());
    let create_portofolio_dto = CreatePortofolioRequestDto {
        project_id,
        judul: portofolio_title.clone(),
        deskripsi: "Desc".to_string(),
        is_active: true,
    };
    let req = test::TestRequest::post()
        .uri("/app/portofolios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_portofolio_dto)
        .to_request();
    let resp: SuccessResponse<PortofolioResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let portofolio_id = resp.data.unwrap().id;

    // 3. Get By ID
    let req = test::TestRequest::get()
        .uri(&format!("/app/portofolios/{}", portofolio_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<PortofolioResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let portofolio = resp.data.unwrap();

    assert_eq!(portofolio.judul, portofolio_title);
    assert_eq!(portofolio.is_active, true); // Default
}

#[actix_web::test]
#[serial]
async fn test_update_portofolio() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // 1. Create Project
    let project_name = format!("Project P4 {}", Utc::now().timestamp_micros());
    let create_project_dto = CreateProjectRequestDto {
        nama_projek: project_name,
        deskripsi: "Desc".to_string(),
        status: "ONGOING".to_string(),
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

    // 2. Create Portofolio
    let portofolio_title = format!("Portofolio 4 {}", Utc::now().timestamp_micros());
    let create_portofolio_dto = CreatePortofolioRequestDto {
        project_id,
        judul: portofolio_title.clone(),
        deskripsi: "Desc".to_string(),
        is_active: true,
    };
    let req = test::TestRequest::post()
        .uri("/app/portofolios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_portofolio_dto)
        .to_request();
    let resp: SuccessResponse<PortofolioResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let portofolio_id = resp.data.unwrap().id;

    // 3. Update
    let update_title = format!("Portofolio Updated {}", Utc::now().timestamp_micros());
    let update_dto = UpdatePortofolioRequestDto {
        project_id: project_id, // Keep existing implicitly by sending same ID
        judul: update_title.clone(),
        deskripsi: "Updated Desc".to_string(),
        is_active: false,
    };
    let req = test::TestRequest::put()
        .uri(&format!("/app/portofolios/{}", portofolio_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&update_dto)
        .to_request();
    let resp: SuccessResponse<PortofolioResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let portofolio = resp.data.unwrap();

    assert_eq!(portofolio.judul, update_title);
    assert_eq!(portofolio.deskripsi, Some("Updated Desc".to_string()));
    assert_eq!(portofolio.is_active, false);

    // 4. Verify (Optional double check via Get, but Update response is trusted now)
    let req = test::TestRequest::get()
        .uri(&format!("/app/portofolios/{}", portofolio_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<PortofolioResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let portofolio_get = resp.data.unwrap();

    assert_eq!(portofolio_get.judul, update_title);
    assert_eq!(portofolio_get.is_active, false);
}

#[actix_web::test]
#[serial]
async fn test_delete_portofolio() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // 1. Create Project
    let project_name = format!("Project P5 {}", Utc::now().timestamp_micros());
    let create_project_dto = CreateProjectRequestDto {
        nama_projek: project_name,
        deskripsi: "Desc".to_string(),
        status: "ONGOING".to_string(),
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

    // 2. Create Portofolio
    let portofolio_title = format!("Portofolio 5 {}", Utc::now().timestamp_micros());
    let create_portofolio_dto = CreatePortofolioRequestDto {
        project_id,
        judul: portofolio_title.clone(),
        deskripsi: "Desc".to_string(),
        is_active: true,
    };
    let req = test::TestRequest::post()
        .uri("/app/portofolios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_portofolio_dto)
        .to_request();
    let resp: SuccessResponse<PortofolioResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let portofolio_id = resp.data.unwrap().id;

    // 3. Delete
    let req = test::TestRequest::delete()
        .uri(&format!("/app/portofolios/{}", portofolio_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // 4. Verify Not Found
    let req = test::TestRequest::get()
        .uri(&format!("/app/portofolios/{}", portofolio_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
}
#[actix_web::test]
#[serial]
async fn test_create_duplicate_portofolio() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // 1. Create Project
    let project_name = format!("Project Dup {}", Utc::now().timestamp_micros());
    let create_project_dto = CreateProjectRequestDto {
        nama_projek: project_name,
        deskripsi: "Desc".to_string(),
        status: "ONGOING".to_string(),
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

    // 2. Create Portofolio 1
    let portofolio_title = format!("Portofolio Dup {}", Utc::now().timestamp_micros());
    let create_portofolio_dto = CreatePortofolioRequestDto {
        project_id,
        judul: portofolio_title.clone(),
        deskripsi: "Portofolio Desc".to_string(),
        is_active: true,
    };

    let req = test::TestRequest::post()
        .uri("/app/portofolios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_portofolio_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // 3. Create Duplicate Portofolio
    let req = test::TestRequest::post()
        .uri("/app/portofolios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_portofolio_dto)
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
    "Portofolio title already exists"
}
