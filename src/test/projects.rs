use crate::app::features::projects::interface::dto::{
    CreateProjectRequestDto, CreateStackRequestDto, ProjectResponseDto, StackResponseDto,
    UpdateProjectRequestDto, UpdateStackRequestDto,
};
use crate::init_test_app;
use crate::test::helpers::{login_admin, seed_user};
use crate::utils::di::Container;
use crate::utils::error_response::ErrorResponse;
use crate::utils::success_response::SuccessResponse;
use actix_web::test;
use chrono::Utc;
use serial_test::serial;

#[actix_web::test]
#[serial]
async fn test_create_stack() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Create Stack
    let stack_name = format!("Rust Create {}", Utc::now().timestamp_micros());
    let create_dto = CreateStackRequestDto {
        nama_stack: stack_name.clone(),
    };

    let req = test::TestRequest::post()
        .uri("/app/stacks")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp: SuccessResponse<StackResponseDto> = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.data.unwrap().nama_stack, stack_name);
}

#[actix_web::test]
#[serial]
async fn test_get_stacks() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Create a stack to ensure list is not empty
    let stack_name = format!("Rust List {}", Utc::now().timestamp_micros());
    let create_dto = CreateStackRequestDto {
        nama_stack: stack_name.clone(),
    };
    let req = test::TestRequest::post()
        .uri("/app/stacks")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    test::call_service(&app, req).await;

    // Get All Stacks
    let req = test::TestRequest::get()
        .uri("/app/stacks")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<
        crate::app::features::projects::interface::dto::PaginatedResponseDto<StackResponseDto>,
    > = test::call_and_read_body_json(&app, req).await;
    let stacks = resp.data.unwrap().items;

    assert!(stacks.len() > 0);
    assert!(stacks.iter().any(|s| s.nama_stack == stack_name));
}

#[actix_web::test]
#[serial]
async fn test_create_project() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Create Stack
    let stack_name = format!("Stack C {}", Utc::now().timestamp_micros());
    let req = test::TestRequest::post()
        .uri("/app/stacks")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&CreateStackRequestDto {
            nama_stack: stack_name,
        })
        .to_request();
    let s1: SuccessResponse<StackResponseDto> = test::call_and_read_body_json(&app, req).await;
    let stack_id = s1.data.unwrap().id;

    // Create Project
    let project_name = format!("Project Create {}", Utc::now().timestamp_micros());
    let create_dto = CreateProjectRequestDto {
        nama_projek: project_name.clone(),
        deskripsi: "Desc".to_string(),
        status: "DRAFT".to_string(),
        progress: 0,
        link_demo: None,
        repository: None,
        tanggal_mulai: "2023-01-01".to_string(),
        tanggal_selesai: None,
        stack_ids: Some(vec![stack_id]),
    };

    let req = test::TestRequest::post()
        .uri("/app/projects")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp: SuccessResponse<ProjectResponseDto> = test::call_and_read_body_json(&app, req).await;
    let project = resp.data.unwrap();

    assert_eq!(project.nama_projek, project_name);
    assert_eq!(project.stacks.len(), 1);
    assert_eq!(project.stacks[0].id, stack_id);
}

#[actix_web::test]
#[serial]
async fn test_get_projects() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Seed project
    let create_dto = CreateProjectRequestDto {
        nama_projek: format!("Project List {}", Utc::now().timestamp_micros()),
        deskripsi: "Desc".to_string(),
        status: "ONGOING".to_string(),
        progress: 10,
        link_demo: None,
        repository: None,
        tanggal_mulai: "2023-01-01".to_string(),
        tanggal_selesai: None,
        stack_ids: None,
    };
    let req = test::TestRequest::post()
        .uri("/app/projects")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    test::call_service(&app, req).await;

    // Get List
    let req = test::TestRequest::get()
        .uri("/app/projects?per_page=100")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<
        crate::app::features::projects::interface::dto::PaginatedResponseDto<ProjectResponseDto>,
    > = test::call_and_read_body_json(&app, req).await;

    assert!(resp.data.unwrap().items.len() > 0);
}

#[actix_web::test]
#[serial]
async fn test_get_project_by_id() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    let project_name = format!("Project GetID {}", Utc::now().timestamp_micros());
    let create_dto = CreateProjectRequestDto {
        nama_projek: project_name.clone(),
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
        .set_json(&create_dto)
        .to_request();
    let resp: SuccessResponse<ProjectResponseDto> = test::call_and_read_body_json(&app, req).await;
    let created_project = resp.data.unwrap();

    let req = test::TestRequest::get()
        .uri(&format!("/app/projects/{}", created_project.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<ProjectResponseDto> = test::call_and_read_body_json(&app, req).await;

    assert_eq!(resp.data.unwrap().nama_projek, project_name);
}

#[actix_web::test]
#[serial]
async fn test_update_project() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    let create_dto = CreateProjectRequestDto {
        nama_projek: format!("Project Update {}", Utc::now().timestamp_micros()),
        deskripsi: "Desc".to_string(),
        status: "DRAFT".to_string(),
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
        .set_json(&create_dto)
        .to_request();
    let resp: SuccessResponse<ProjectResponseDto> = test::call_and_read_body_json(&app, req).await;
    let id = resp.data.unwrap().id;

    let updated_name = format!("Updated Name {}", Utc::now().timestamp_micros());
    let update_dto = UpdateProjectRequestDto {
        nama_projek: updated_name.clone(),
        deskripsi: "Desc".to_string(),
        status: "COMPLETED".to_string(),
        progress: Some(100),
        link_demo: None,
        repository: None,
        tanggal_mulai: "2023-01-01".to_string(),
        tanggal_selesai: None,
        stack_ids: None,
    };

    let req = test::TestRequest::put()
        .uri(&format!("/app/projects/{}", id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&update_dto)
        .to_request();
    let resp: SuccessResponse<ProjectResponseDto> = test::call_and_read_body_json(&app, req).await;
    let updated = resp.data.unwrap();

    assert_eq!(updated.nama_projek, updated_name);
    assert_eq!(updated.status, "COMPLETED");
    assert_eq!(updated.progress, 100);
}

#[actix_web::test]
#[serial]
async fn test_delete_project() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    let create_dto = CreateProjectRequestDto {
        nama_projek: format!("Project Delete {}", Utc::now().timestamp_micros()),
        deskripsi: "Desc".to_string(),
        status: "DRAFT".to_string(),
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
        .set_json(&create_dto)
        .to_request();
    let resp: SuccessResponse<ProjectResponseDto> = test::call_and_read_body_json(&app, req).await;
    let id = resp.data.unwrap().id;

    // Delete
    let req = test::TestRequest::delete()
        .uri(&format!("/app/projects/{}", id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Verify Not Found
    let req = test::TestRequest::get()
        .uri(&format!("/app/projects/{}", id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
}

#[actix_web::test]
#[serial]
async fn test_project_stack_flow() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // 1. Create Stacks (Path: Create Stack)
    let stack1_name = format!("Flow Stack 1 {}", Utc::now().timestamp_micros());
    let stack2_name = format!("Flow Stack 2 {}", Utc::now().timestamp_micros());

    let req = test::TestRequest::post()
        .uri("/app/stacks")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&CreateStackRequestDto {
            nama_stack: stack1_name.clone(),
        })
        .to_request();
    let s1: SuccessResponse<StackResponseDto> = test::call_and_read_body_json(&app, req).await;
    let stack1_id = s1.data.unwrap().id;

    let req = test::TestRequest::post()
        .uri("/app/stacks")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&CreateStackRequestDto {
            nama_stack: stack2_name.clone(),
        })
        .to_request();
    let s2: SuccessResponse<StackResponseDto> = test::call_and_read_body_json(&app, req).await;
    let stack2_id = s2.data.unwrap().id;

    // 2. Create Project with Stacks (Path: Create Project + Relation)
    let project_name = format!("Flow Project {}", Utc::now().timestamp_micros());
    let create_dto = CreateProjectRequestDto {
        nama_projek: project_name.clone(),
        deskripsi: "Flow Desc".to_string(),
        status: "ONGOING".to_string(),
        progress: 50,
        link_demo: None,
        repository: None,
        tanggal_mulai: "2023-01-01".to_string(),
        tanggal_selesai: None,
        stack_ids: Some(vec![stack1_id, stack2_id]),
    };

    let req = test::TestRequest::post()
        .uri("/app/projects")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp: SuccessResponse<ProjectResponseDto> = test::call_and_read_body_json(&app, req).await;
    let project = resp.data.unwrap();

    assert_eq!(project.stacks.len(), 2);
    let project_id = project.id;

    // 3. Update Project Stacks (Path: Update Project + Relation)
    // Remove stack2, keep stack1
    let update_dto = UpdateProjectRequestDto {
        nama_projek: project_name.clone(), // Keep original name
        deskripsi: "Flow Desc".to_string(),
        status: "COMPLETED".to_string(),
        progress: Some(100),
        link_demo: None,
        repository: None,
        tanggal_mulai: "2023-01-01".to_string(),
        tanggal_selesai: None,
        stack_ids: Some(vec![stack1_id]),
    };

    let req = test::TestRequest::put()
        .uri(&format!("/app/projects/{}", project_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&update_dto)
        .to_request();
    let resp: SuccessResponse<ProjectResponseDto> = test::call_and_read_body_json(&app, req).await;
    let updated = resp.data.unwrap();

    assert_eq!(updated.stacks.len(), 1);
    assert_eq!(updated.stacks[0].id, stack1_id);
    assert_eq!(updated.status, "COMPLETED");
}

#[actix_web::test]
#[serial]
async fn test_get_stack_by_id() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Create Stack
    let stack_name = format!("Rust GetID {}", Utc::now().timestamp_micros());
    let create_dto = CreateStackRequestDto {
        nama_stack: stack_name.clone(),
    };
    let req = test::TestRequest::post()
        .uri("/app/stacks")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp: SuccessResponse<StackResponseDto> = test::call_and_read_body_json(&app, req).await;
    let stack_id = resp.data.unwrap().id;

    // Get Stack By ID
    let req = test::TestRequest::get()
        .uri(&format!("/app/stacks/{}", stack_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<StackResponseDto> = test::call_and_read_body_json(&app, req).await;

    assert_eq!(resp.data.unwrap().nama_stack, stack_name);
}

#[actix_web::test]
#[serial]
async fn test_update_stack() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Create Stack
    let stack_name = format!("Rust Update {}", Utc::now().timestamp_micros());
    let create_dto = CreateStackRequestDto {
        nama_stack: stack_name.clone(),
    };
    let req = test::TestRequest::post()
        .uri("/app/stacks")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp: SuccessResponse<StackResponseDto> = test::call_and_read_body_json(&app, req).await;
    let stack_id = resp.data.unwrap().id;

    // Update Stack
    let update_name = format!("Rust Updated {}", Utc::now().timestamp_micros());
    let update_dto = UpdateStackRequestDto {
        nama_stack: update_name.clone(),
    };
    let req = test::TestRequest::put()
        .uri(&format!("/app/stacks/{}", stack_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&update_dto)
        .to_request();
    let resp: SuccessResponse<StackResponseDto> = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.data.unwrap().nama_stack, update_name);
}

#[actix_web::test]
#[serial]
async fn test_delete_stack() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Create Stack
    let stack_name = format!("Rust Delete {}", Utc::now().timestamp_micros());
    let create_dto = CreateStackRequestDto {
        nama_stack: stack_name.clone(),
    };
    let req = test::TestRequest::post()
        .uri("/app/stacks")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp: SuccessResponse<StackResponseDto> = test::call_and_read_body_json(&app, req).await;
    let stack_id = resp.data.unwrap().id;

    // Delete Stack
    let req = test::TestRequest::delete()
        .uri(&format!("/app/stacks/{}", stack_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Verify Not Found
    let req = test::TestRequest::get()
        .uri(&format!("/app/stacks/{}", stack_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
}

#[actix_web::test]
#[serial]
async fn test_create_duplicate_project() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    let project_name = format!("Unique Proj {}", Utc::now().timestamp_micros());
    let create_dto = CreateProjectRequestDto {
        nama_projek: project_name.clone(),
        deskripsi: "Desc".to_string(),
        status: "DRAFT".to_string(),
        progress: 0,
        link_demo: None,
        repository: None,
        tanggal_mulai: "2023-01-01".to_string(),
        tanggal_selesai: None,
        stack_ids: None,
    };

    // First Create (Success)
    let req = test::TestRequest::post()
        .uri("/app/projects")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Second Create (Fail)
    let req = test::TestRequest::post()
        .uri("/app/projects")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);

    // Check error message
    let body: ErrorResponse = test::read_body_json(resp).await;
    assert!(body.errors.is_some());
    assert_eq!(
        body.errors.unwrap().get("nama_projek").unwrap(),
        "Project name already exists"
    );
}

#[actix_web::test]
#[serial]
async fn test_create_duplicate_stack() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    let stack_name = format!("Unique Stack {}", Utc::now().timestamp_micros());
    let create_dto = CreateStackRequestDto {
        nama_stack: stack_name.clone(),
    };

    // First Create (Success)
    let req = test::TestRequest::post()
        .uri("/app/stacks")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Second Create (Fail)
    let req = test::TestRequest::post()
        .uri("/app/stacks")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Verify error message
    let body: ErrorResponse = test::read_body_json(resp).await;
    assert_eq!(body.message, "Validation Error");
    assert!(body.errors.is_some());
    assert_eq!(
        body.errors.unwrap().get("nama_stack").unwrap(),
        "Stack name already exists"
    );
}

#[actix_web::test]
#[serial]
async fn test_update_stack_duplicate_name() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Create Stack 1
    let stack1_name = format!("Stack 1 {}", Utc::now().timestamp_micros());
    let req = test::TestRequest::post()
        .uri("/app/stacks")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&CreateStackRequestDto {
            nama_stack: stack1_name.clone(),
        })
        .to_request();
    let resp: SuccessResponse<StackResponseDto> = test::call_and_read_body_json(&app, req).await;
    let _stack1_id = resp.data.unwrap().id;

    // Create Stack 2
    let stack2_name = format!("Stack 2 {}", Utc::now().timestamp_micros());
    let req = test::TestRequest::post()
        .uri("/app/stacks")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&CreateStackRequestDto {
            nama_stack: stack2_name.clone(),
        })
        .to_request();
    let resp: SuccessResponse<StackResponseDto> = test::call_and_read_body_json(&app, req).await;
    let stack2_id = resp.data.unwrap().id;

    // Update Stack 2 with Stack 1's name (Should Fail)
    let update_dto = UpdateStackRequestDto {
        nama_stack: stack1_name.clone(),
    };

    let req = test::TestRequest::put()
        .uri(&format!("/app/stacks/{}", stack2_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&update_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Currently this will PASS (status success) because we haven't implemented the check yet.
    // So we assert failure to demonstrate the "bug" (missing feature).
    // Once fixed, this assert should pass (response is failure).

    // Verify error message
    let body: ErrorResponse = test::read_body_json(resp).await;
    assert_eq!(body.message, "Validation Error");
    assert!(body.errors.is_some());
    assert_eq!(
        body.errors.unwrap().get("nama_stack").unwrap(),
        "Stack name already exists"
    );
}
