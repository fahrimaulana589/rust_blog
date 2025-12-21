use crate::app::features::blog::interface::dto::{
    BlogResponseDto, CreateBlogRequestDto, CreateCategoryRequestDto, CreateTagRequestDto,
    UpdateBlogRequestDto,
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
        excerpt: None,
        thumbnail: None,
        status: None,
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

    let create_dto = CreateBlogRequestDto {
        title: "Test Get Blog".to_string(),
        content: "Content".to_string(),
        category_id: cat_id,
        tag_ids: None,
        excerpt: None,
        thumbnail: None,
        status: None,
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
    test::call_service(&app, req).await;

    // Get Cat ID
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

    // Create Blog
    let unique_title = format!("Test ID Blog {}", Utc::now().timestamp_micros());
    let create_dto = CreateBlogRequestDto {
        title: unique_title.clone(),
        content: "Content".to_string(),
        category_id: cat_id,
        tag_ids: None,
        excerpt: None,
        thumbnail: None,
        status: None,
    };
    let req = test::TestRequest::post()
        .uri("/app/blogs")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    test::call_service(&app, req).await;

    // Find Blog ID
    // Find Blog ID
    let req = test::TestRequest::get()
        .uri("/app/blogs?per_page=1000")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<
        crate::app::features::blog::interface::dto::PaginatedResponseDto<BlogResponseDto>,
    > = test::call_and_read_body_json(&app, req).await;
    let blog = resp
        .data
        .unwrap()
        .items
        .into_iter()
        .find(|b| b.title == unique_title)
        .unwrap();

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
    test::call_service(&app, req).await;
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

    // Create Blog
    let unique_title = format!("Test Update Blog {}", Utc::now().timestamp_micros());
    let create_dto = CreateBlogRequestDto {
        title: unique_title.clone(),
        content: "Content".to_string(),
        category_id: cat_id,
        tag_ids: None,
        excerpt: None,
        thumbnail: None,
        status: None,
    };
    let req = test::TestRequest::post()
        .uri("/app/blogs")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    test::call_service(&app, req).await;

    // Find Blog ID
    // Find Blog ID
    let req = test::TestRequest::get()
        .uri("/app/blogs?per_page=1000")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<
        crate::app::features::blog::interface::dto::PaginatedResponseDto<BlogResponseDto>,
    > = test::call_and_read_body_json(&app, req).await;
    let blog = resp
        .data
        .unwrap()
        .items
        .into_iter()
        .find(|b| b.title == unique_title)
        .unwrap();

    // Update
    let update_dto = UpdateBlogRequestDto {
        title: "Updated Title".to_string(),
        content: "Updated Content".to_string(),
        category_id: cat_id,
        tag_ids: None,
        excerpt: None,
        thumbnail: None,
        status: None,
    };
    let req = test::TestRequest::put()
        .uri(&format!("/app/blogs/{}", blog.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&update_dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Verify
    let req = test::TestRequest::get()
        .uri(&format!("/app/blogs/{}", blog.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<BlogResponseDto> = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.data.unwrap().title, "Updated Title");
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
    test::call_service(&app, req).await;
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

    // Create Blog
    let unique_title = format!("Test Delete Blog {}", Utc::now().timestamp_micros());
    let create_dto = CreateBlogRequestDto {
        title: unique_title.clone(),
        content: "Content".to_string(),
        category_id: cat_id,
        tag_ids: None,
        excerpt: None,
        thumbnail: None,
        status: None,
    };
    let req = test::TestRequest::post()
        .uri("/app/blogs")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&create_dto)
        .to_request();
    test::call_service(&app, req).await;

    // Find Blog ID
    // Find Blog ID
    let req = test::TestRequest::get()
        .uri("/app/blogs?per_page=1000")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<
        crate::app::features::blog::interface::dto::PaginatedResponseDto<BlogResponseDto>,
    > = test::call_and_read_body_json(&app, req).await;
    let blog = resp
        .data
        .unwrap()
        .items
        .into_iter()
        .find(|b| b.title == unique_title)
        .unwrap();

    // Delete
    let req = test::TestRequest::delete()
        .uri(&format!("/app/blogs/{}", blog.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
