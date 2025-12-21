use crate::app::features::blog::interface::dto::{
    CreateTagRequestDto, TagResponseDto, UpdateTagRequestDto,
};
use crate::init_test_app;
use crate::test::helpers::{login_admin, seed_user};
use crate::utils::di::Container;
use crate::utils::success_response::SuccessResponse;
use actix_web::test;
use chrono::Utc;
use serial_test::serial;

#[actix_web::test]
#[serial]
async fn test_create_tag() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    let create_dto = CreateTagRequestDto {
        name: format!("Test Create Tag {}", Utc::now().timestamp_micros()),
    };

    let req = test::TestRequest::post()
        .uri("/app/tags")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
#[serial]
async fn test_get_tags() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Create a tag
    let create_dto = CreateTagRequestDto {
        name: format!("Test Get Tags {}", Utc::now().timestamp_micros()),
    };
    let req = test::TestRequest::post()
        .uri("/app/tags")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    test::call_service(&app, req).await;

    let req = test::TestRequest::get()
        .uri("/app/tags?per_page=1000")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<
        crate::app::features::blog::interface::dto::PaginatedResponseDto<TagResponseDto>,
    > = test::call_and_read_body_json(&app, req).await;
    assert!(resp.data.unwrap().items.len() > 0);
}

#[actix_web::test]
#[serial]
async fn test_get_tag_by_id() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Create
    let unique_name = format!("Test GetID Tag {}", Utc::now().timestamp_micros());
    let create_dto = CreateTagRequestDto {
        name: unique_name.clone(),
    };
    let req = test::TestRequest::post()
        .uri("/app/tags")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    test::call_service(&app, req).await;

    // Get All to find ID
    let req = test::TestRequest::get()
        .uri("/app/tags?per_page=1000")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<
        crate::app::features::blog::interface::dto::PaginatedResponseDto<TagResponseDto>,
    > = test::call_and_read_body_json(&app, req).await;
    let tag = resp
        .data
        .unwrap()
        .items
        .into_iter()
        .find(|t| t.name == unique_name)
        .expect("Tag not found");

    // Get by ID
    let req = test::TestRequest::get()
        .uri(&format!("/app/tags/{}", tag.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<TagResponseDto> = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.data.unwrap().name, unique_name);
}

#[actix_web::test]
#[serial]
async fn test_update_tag() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Create
    let unique_name = format!("Test Update Tag {}", Utc::now().timestamp_micros());
    let create_dto = CreateTagRequestDto {
        name: unique_name.clone(),
    };
    let req = test::TestRequest::post()
        .uri("/app/tags")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    test::call_service(&app, req).await;

    // Get ID
    let req = test::TestRequest::get()
        .uri("/app/tags?per_page=1000")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<
        crate::app::features::blog::interface::dto::PaginatedResponseDto<TagResponseDto>,
    > = test::call_and_read_body_json(&app, req).await;
    let tag = resp
        .data
        .unwrap()
        .items
        .into_iter()
        .find(|t| t.name == unique_name)
        .expect("Tag not found");

    // Update
    let update_dto = UpdateTagRequestDto {
        name: Some("Test Updated Tag Name".to_string()),
    };
    let req = test::TestRequest::put()
        .uri(&format!("/app/tags/{}", tag.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&update_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Verify
    let req = test::TestRequest::get()
        .uri(&format!("/app/tags/{}", tag.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<TagResponseDto> = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.data.unwrap().name, "Test Updated Tag Name");
}

#[actix_web::test]
#[serial]
async fn test_delete_tag() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Create
    let unique_name = format!("Test Delete Tag {}", Utc::now().timestamp_micros());
    let create_dto = CreateTagRequestDto {
        name: unique_name.clone(),
    };
    let req = test::TestRequest::post()
        .uri("/app/tags")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    test::call_service(&app, req).await;

    // Get ID
    let req = test::TestRequest::get()
        .uri("/app/tags?per_page=1000")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<
        crate::app::features::blog::interface::dto::PaginatedResponseDto<TagResponseDto>,
    > = test::call_and_read_body_json(&app, req).await;
    let tag = resp
        .data
        .unwrap()
        .items
        .into_iter()
        .find(|t| t.name == unique_name)
        .expect("Tag not found");

    // Delete
    let req = test::TestRequest::delete()
        .uri(&format!("/app/tags/{}", tag.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
