use crate::app::features::portofolio::interface::dto::{
    CreatePortofolioRequestDto, PortofolioResponseDto,
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
async fn test_get_portofolio_by_slug() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // 1. Create Project
    let project_name = format!("Project Slug {}", Utc::now().timestamp_micros());
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
    let portofolio_title = format!("Portofolio Slug {}", Utc::now().timestamp_micros());
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
    let portofolio = resp.data.unwrap();
    let slug = portofolio.slug;

    // 3. Get By Slug
    let req = test::TestRequest::get()
        .uri(&format!("/app/portofolios/slug/{}", slug))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<PortofolioResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let portofolio_resp = resp.data.unwrap();

    assert_eq!(portofolio_resp.judul, portofolio_title);
    assert_eq!(portofolio_resp.project.id, project_id);
}

#[actix_web::test]
#[serial]
async fn test_get_portofolio_by_slug_not_found() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    let req = test::TestRequest::get()
        .uri("/app/portofolios/slug/non-existent-portfolio-slug")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
}
