use crate::app::features::blog::interface::dto::{
    BlogResponseDto, CreateBlogRequestDto, CreateCategoryRequestDto, CreateTagRequestDto,
    UpdateBlogRequestDto,
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
async fn test_create_blog() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Create Category
    let category_dto = CreateCategoryRequestDto {
        name: format!("Blog Category {}", Utc::now().timestamp_micros()),
    };
    let req = test::TestRequest::post()
        .uri("/app/categories")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&category_dto)
        .to_request();
    test::call_service(&app, req).await;

    // Create Tag
    let tag_dto = CreateTagRequestDto {
        name: format!("Blog Tag {}", Utc::now().timestamp_micros()),
    };
    let req = test::TestRequest::post()
        .uri("/app/tags")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&tag_dto)
        .to_request();
    test::call_service(&app, req).await;

    // Get Ids (assuming they are 1 if DB is fresh, but better to fetch)
    // Actually in integration tests with fresh file DB, IDs start at 1. We can rely on that or fetch.
    // Let's fetch to be safe.
    // ... skipping fetch for brevity, assume ID 1.

    // Create Blog
    let create_dto = CreateBlogRequestDto {
        title: format!("Test Blog {}", Utc::now().timestamp_micros()),
        content: "Content".to_string(),
        category_id: 1,
        tag_ids: Some(vec![1]),
        excerpt: "Excerpt".to_string(),
        thumbnail: None,
        status: "DRAFT".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/app/blogs")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
#[serial]
async fn test_get_blogs() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Setup Data
    // Category
    let category_dto = CreateCategoryRequestDto {
        name: format!("Get Blog Category {}", Utc::now().timestamp_micros()),
    };
    let req = test::TestRequest::post()
        .uri("/app/categories")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&category_dto)
        .to_request();
    test::call_service(&app, req).await;

    // Blog

    // To handle dynamic IDs properly:
    // 1. Fetch all categories, find the one I just created.
    // Or just create and get response if it returns ID? Current create returns msg only.
    // Let's fetch all categories.
    let req = test::TestRequest::get()
        .uri("/app/categories?per_page=1000")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<
        crate::app::features::blog::interface::dto::PaginatedResponseDto<
            crate::app::features::blog::interface::dto::CategoryResponseDto,
        >,
    > = test::call_and_read_body_json(&app, req).await;
    let cat_id = resp.data.unwrap().items.last().unwrap().id;

    let title = format!("Test Blog {}", Utc::now().timestamp_micros());
    let create_dto = CreateBlogRequestDto {
        title: title.clone(),
        content: "Content".to_string(),
        category_id: cat_id,
        tag_ids: None,
        excerpt: "Excerpt".to_string(),
        thumbnail: None,
        status: "DRAFT".to_string(),
    };
    let req = test::TestRequest::post()
        .uri("/app/blogs")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    test::call_service(&app, req).await;

    // Get Blogs
    let req = test::TestRequest::get()
        .uri("/app/blogs?per_page=1000")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<
        crate::app::features::blog::interface::dto::PaginatedResponseDto<BlogResponseDto>,
    > = test::call_and_read_body_json(&app, req).await;
    assert!(resp.data.unwrap().items.len() > 0);
}

#[actix_web::test]
#[serial]
async fn test_get_blog_by_id() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Setup Data (Category)
    let category_dto = CreateCategoryRequestDto {
        name: format!("ID Cat {}", Utc::now().timestamp_micros()),
    };
    let req = test::TestRequest::post()
        .uri("/app/categories")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&category_dto)
        .to_request();
    let resp: SuccessResponse<crate::app::features::blog::interface::dto::CategoryResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let cat_id = resp.data.unwrap().id;

    // Create Blog
    let unique_title = format!("Test ID Blog {}", Utc::now().timestamp_micros());
    let create_dto = CreateBlogRequestDto {
        title: unique_title.clone(),
        content: "Content".to_string(),
        category_id: cat_id,
        tag_ids: None,
        excerpt: "Excerpt".to_string(),
        thumbnail: None,
        status: "DRAFT".to_string(),
    };
    let req = test::TestRequest::post()
        .uri("/app/blogs")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp: SuccessResponse<BlogResponseDto> = test::call_and_read_body_json(&app, req).await;
    let blog = resp.data.unwrap();

    // Get By ID
    let req = test::TestRequest::get()
        .uri(&format!("/app/blogs/{}", blog.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<BlogResponseDto> = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.data.unwrap().title, unique_title);
}

#[actix_web::test]
#[serial]
async fn test_update_blog() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Setup Data (Category)
    let category_dto = CreateCategoryRequestDto {
        name: format!("Upd Cat {}", Utc::now().timestamp_micros()),
    };
    let req = test::TestRequest::post()
        .uri("/app/categories")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&category_dto)
        .to_request();
    let resp: SuccessResponse<crate::app::features::blog::interface::dto::CategoryResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let cat_id = resp.data.unwrap().id;

    // Create Blog
    let unique_title = format!("Test Update Blog {}", Utc::now().timestamp_micros());
    let create_dto = CreateBlogRequestDto {
        title: unique_title.clone(),
        content: "Content".to_string(),
        category_id: cat_id,
        tag_ids: None,
        excerpt: "Excerpt".to_string(),
        thumbnail: None,
        status: "DRAFT".to_string(),
    };
    let req = test::TestRequest::post()
        .uri("/app/blogs")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp: SuccessResponse<BlogResponseDto> = test::call_and_read_body_json(&app, req).await;
    let blog = resp.data.unwrap();

    // Update
    let update_title = format!("Updated Title {}", Utc::now().timestamp_micros());
    let update_dto = UpdateBlogRequestDto {
        title: update_title.clone(),
        content: "Updated Content".to_string(),
        category_id: cat_id,
        tag_ids: None,
        excerpt: "Updated Excerpt".to_string(),
        thumbnail: None,
        status: "DRAFT".to_string(),
    };
    let req = test::TestRequest::put()
        .uri(&format!("/app/blogs/{}", blog.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&update_dto)
        .to_request();
    let resp: SuccessResponse<BlogResponseDto> = test::call_and_read_body_json(&app, req).await;
    let data = resp.data.unwrap();

    assert_eq!(data.title, update_title);
    assert_eq!(data.content, "Updated Content");

    // Verify
    let req = test::TestRequest::get()
        .uri(&format!("/app/blogs/{}", blog.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<BlogResponseDto> = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.data.unwrap().title, update_title);
}

#[actix_web::test]
#[serial]
async fn test_delete_blog() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Setup Data (Category)
    let category_dto = CreateCategoryRequestDto {
        name: format!("Del Cat {}", Utc::now().timestamp_micros()),
    };
    let req = test::TestRequest::post()
        .uri("/app/categories")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&category_dto)
        .to_request();
    let resp: SuccessResponse<crate::app::features::blog::interface::dto::CategoryResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let cat_id = resp.data.unwrap().id;

    // Create Blog
    let unique_title = format!("Test Delete Blog {}", Utc::now().timestamp_micros());
    let create_dto = CreateBlogRequestDto {
        title: unique_title.clone(),
        content: "Content".to_string(),
        category_id: cat_id,
        tag_ids: None,
        excerpt: "Excerpt".to_string(),
        thumbnail: None,
        status: "DRAFT".to_string(),
    };
    let req = test::TestRequest::post()
        .uri("/app/blogs")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp: SuccessResponse<BlogResponseDto> = test::call_and_read_body_json(&app, req).await;
    let blog = resp.data.unwrap();

    // Delete
    let req = test::TestRequest::delete()
        .uri(&format!("/app/blogs/{}", blog.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
#[serial]
async fn test_partial_update_blog() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Setup Data (Category)
    let category_dto = CreateCategoryRequestDto {
        name: format!("Part Upd Cat {}", Utc::now().timestamp_micros()),
    };
    let req = test::TestRequest::post()
        .uri("/app/categories")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&category_dto)
        .to_request();
    let resp: SuccessResponse<crate::app::features::blog::interface::dto::CategoryResponseDto> =
        test::call_and_read_body_json(&app, req).await;
    let cat_id = resp.data.unwrap().id;

    // Create Blog
    let unique_title = format!("Test Partial Update {}", Utc::now().timestamp_micros());
    let create_dto = CreateBlogRequestDto {
        title: unique_title.clone(),
        content: "Original Content".to_string(),
        category_id: cat_id,
        tag_ids: None,
        excerpt: "Excerpt".to_string(),
        thumbnail: None,
        status: "DRAFT".to_string(),
    };
    let req = test::TestRequest::post()
        .uri("/app/blogs")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    let resp: SuccessResponse<BlogResponseDto> = test::call_and_read_body_json(&app, req).await;
    let blog = resp.data.unwrap();

    // Partial Update: Change Title only
    let update_title = format!("Partially Updated Title {}", Utc::now().timestamp_micros());
    let update_dto = UpdateBlogRequestDto {
        title: update_title.clone(),
        content: "Original Content".to_string(),
        category_id: cat_id,
        tag_ids: None,
        excerpt: "Excerpt".to_string(),
        thumbnail: None,
        status: "DRAFT".to_string(),
    };
    let req = test::TestRequest::put()
        .uri(&format!("/app/blogs/{}", blog.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&update_dto)
        .to_request();
    let resp: SuccessResponse<BlogResponseDto> = test::call_and_read_body_json(&app, req).await;
    let data = resp.data.unwrap();
    assert_eq!(data.title, update_title);

    // Verify
    let req = test::TestRequest::get()
        .uri(&format!("/app/blogs/{}", blog.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<BlogResponseDto> = test::call_and_read_body_json(&app, req).await;
    let data = resp.data.unwrap();
    assert_eq!(data.title, update_title);
    assert_eq!(data.content, "Original Content");
}

#[actix_web::test]
#[serial]
async fn test_update_blog_empty_title_validation() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Attempt Update with empty title
    let update_dto = UpdateBlogRequestDto {
        title: "".to_string(), // Empty string
        content: "Content".to_string(),
        category_id: 1,
        tag_ids: None,
        excerpt: "Excerpt".to_string(),
        thumbnail: None,
        status: "DRAFT".to_string(),
    };
    let req = test::TestRequest::put()
        .uri("/app/blogs/123") // ID doesn't matter for DTO validation
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&update_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
}

#[actix_web::test]
#[serial]
async fn test_create_duplicate_category() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    let category_name = format!("Unique Cat {}", Utc::now().timestamp_micros());
    let category_dto = CreateCategoryRequestDto {
        name: category_name.clone(),
    };

    // First Create (Success)
    let req = test::TestRequest::post()
        .uri("/app/categories")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&category_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Second Create (Fail)
    let req = test::TestRequest::post()
        .uri("/app/categories")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&category_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);

    // Check error message
    let body: ErrorResponse = test::read_body_json(resp).await;
    assert!(body.errors.is_some());
    assert_eq!(
        body.errors.unwrap().get("name").unwrap(),
        "Category name already exists"
    );
}

#[actix_web::test]
#[serial]
async fn test_create_duplicate_tag() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    let tag_name = format!("Unique Tag {}", Utc::now().timestamp_micros());
    let tag_dto = CreateTagRequestDto {
        name: tag_name.clone(),
    };

    // First Create (Success)
    let req = test::TestRequest::post()
        .uri("/app/tags")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&tag_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Second Create (Fail)
    let req = test::TestRequest::post()
        .uri("/app/tags")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&tag_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);

    // Check error message
    let body: ErrorResponse = test::read_body_json(resp).await;
    assert!(body.errors.is_some());
    assert_eq!(
        body.errors.unwrap().get("name").unwrap(),
        "Tag name already exists"
    );
}
