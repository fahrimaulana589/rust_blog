use crate::app::features::blog::interface::dto::{
    CategoryResponseDto, CreateCategoryRequestDto, UpdateCategoryRequestDto,
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
async fn test_create_category() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    let create_dto = CreateCategoryRequestDto {
        name: format!("Test Create Category {}", Utc::now().timestamp_micros()),
    };

    let req = test::TestRequest::post()
        .uri("/app/categories")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
#[serial]
async fn test_get_categories() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Create a category to ensure at least one exists
    let create_dto = CreateCategoryRequestDto {
        name: format!("Test Get Category {}", Utc::now().timestamp_micros()),
    };
    let req = test::TestRequest::post()
        .uri("/app/categories")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    test::call_service(&app, req).await;

    let req = test::TestRequest::get()
        .uri("/app/categories?per_page=1000")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<
        crate::app::features::blog::interface::dto::PaginatedResponseDto<CategoryResponseDto>,
    > = test::call_and_read_body_json(&app, req).await;
    assert!(resp.data.unwrap().items.len() > 0);
}

#[actix_web::test]
#[serial]
async fn test_get_category_by_id() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Create
    let unique_name = format!("Test GetID Category {}", Utc::now().timestamp_micros());
    let create_dto = CreateCategoryRequestDto {
        name: unique_name.clone(),
    };
    let req = test::TestRequest::post()
        .uri("/app/categories")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Retrieve to get ID
    let req = test::TestRequest::get()
        .uri("/app/categories?per_page=1000")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<
        crate::app::features::blog::interface::dto::PaginatedResponseDto<CategoryResponseDto>,
    > = test::call_and_read_body_json(&app, req).await;
    let category = resp
        .data
        .unwrap()
        .items
        .into_iter()
        .find(|c| c.name == unique_name)
        .expect("Category not found");

    // Get by ID
    let req = test::TestRequest::get()
        .uri(&format!("/app/categories/{}", category.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<CategoryResponseDto> = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.data.unwrap().name, unique_name);
}

#[actix_web::test]
#[serial]
async fn test_update_category() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Create
    let unique_name = format!("Test Update Category {}", Utc::now().timestamp_micros());
    let create_dto = CreateCategoryRequestDto {
        name: unique_name.clone(),
    };
    let req = test::TestRequest::post()
        .uri("/app/categories")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Get ID
    let req = test::TestRequest::get()
        .uri("/app/categories?per_page=1000")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<
        crate::app::features::blog::interface::dto::PaginatedResponseDto<CategoryResponseDto>,
    > = test::call_and_read_body_json(&app, req).await;
    let category = resp
        .data
        .unwrap()
        .items
        .into_iter()
        .find(|c| c.name == unique_name)
        .expect("Category not found");

    // Update
    let update_dto = UpdateCategoryRequestDto {
        name: Some("Test Updated Name".to_string()),
    };
    let req = test::TestRequest::put()
        .uri(&format!("/app/categories/{}", category.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&update_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Verify
    let req = test::TestRequest::get()
        .uri(&format!("/app/categories/{}", category.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<CategoryResponseDto> = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.data.unwrap().name, "Test Updated Name");
}

#[actix_web::test]
#[serial]
async fn test_delete_category() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Create
    let unique_name = format!("Test Delete Category {}", Utc::now().timestamp_micros());
    let create_dto = CreateCategoryRequestDto {
        name: unique_name.clone(),
    };
    let req = test::TestRequest::post()
        .uri("/app/categories")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Get ID
    let req = test::TestRequest::get()
        .uri("/app/categories?per_page=1000")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<
        crate::app::features::blog::interface::dto::PaginatedResponseDto<CategoryResponseDto>,
    > = test::call_and_read_body_json(&app, req).await;
    let category = resp
        .data
        .unwrap()
        .items
        .into_iter()
        .find(|c| c.name == unique_name)
        .expect("Category not found");

    // Delete
    let req = test::TestRequest::delete()
        .uri(&format!("/app/categories/{}", category.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
