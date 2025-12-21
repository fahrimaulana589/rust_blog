use crate::app::features::blog::interface::dto::{
    CreateTagRequestDto, TagResponseDto, UpdateTagRequestDto,
};
use crate::init_test_app;
use crate::test::helpers::{login_admin, seed_user};
use crate::utils::di::Container;
use crate::utils::success_response::SuccessResponse;
use actix_web::test;
use serial_test::serial;

#[actix_web::test]
#[serial]
async fn test_create_tag() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    let create_dto = CreateTagRequestDto {
        name: "Test Create Tag".to_string(),
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
        name: "Test Get Tags".to_string(),
    };
    let req = test::TestRequest::post()
        .uri("/app/tags")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    test::call_service(&app, req).await;

    let req = test::TestRequest::get()
        .uri("/app/tags")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<Vec<TagResponseDto>> = test::call_and_read_body_json(&app, req).await;
    assert!(resp.data.unwrap().len() > 0);
}

#[actix_web::test]
#[serial]
async fn test_get_tag_by_id() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Create
    let create_dto = CreateTagRequestDto {
        name: "Test GetID Tag".to_string(),
    };
    let req = test::TestRequest::post()
        .uri("/app/tags")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    test::call_service(&app, req).await;

    // Get All to find ID
    let req = test::TestRequest::get()
        .uri("/app/tags")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<Vec<TagResponseDto>> = test::call_and_read_body_json(&app, req).await;
    let tag = resp
        .data
        .unwrap()
        .into_iter()
        .find(|t| t.name == "Test GetID Tag")
        .expect("Tag not found");

    // Get by ID
    let req = test::TestRequest::get()
        .uri(&format!("/app/tags/{}", tag.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<TagResponseDto> = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.data.unwrap().name, "Test GetID Tag");
}

#[actix_web::test]
#[serial]
async fn test_update_tag() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Create
    let create_dto = CreateTagRequestDto {
        name: "Test Update Tag".to_string(),
    };
    let req = test::TestRequest::post()
        .uri("/app/tags")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    test::call_service(&app, req).await;

    // Get ID
    let req = test::TestRequest::get()
        .uri("/app/tags")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<Vec<TagResponseDto>> = test::call_and_read_body_json(&app, req).await;
    let tag = resp
        .data
        .unwrap()
        .into_iter()
        .find(|t| t.name == "Test Update Tag")
        .expect("Tag not found");

    // Update
    let update_dto = UpdateTagRequestDto {
        name: "Test Updated Tag Name".to_string(),
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
    let create_dto = CreateTagRequestDto {
        name: "Test Delete Tag".to_string(),
    };
    let req = test::TestRequest::post()
        .uri("/app/tags")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    test::call_service(&app, req).await;

    // Get ID
    let req = test::TestRequest::get()
        .uri("/app/tags")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<Vec<TagResponseDto>> = test::call_and_read_body_json(&app, req).await;
    let tag = resp
        .data
        .unwrap()
        .into_iter()
        .find(|t| t.name == "Test Delete Tag")
        .expect("Tag not found");

    // Delete
    let req = test::TestRequest::delete()
        .uri(&format!("/app/tags/{}", tag.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
