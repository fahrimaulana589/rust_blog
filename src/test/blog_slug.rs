use crate::app::features::blog::interface::dto::{
    BlogResponseDto, CreateBlogRequestDto, CreateCategoryRequestDto,
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
async fn test_get_blog_by_slug() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Setup Data (Category)
    let category_dto = CreateCategoryRequestDto {
        name: format!("Slug Cat {}", Utc::now().timestamp_micros()),
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
    let unique_title = format!("Test Slug Blog {}", Utc::now().timestamp_micros());
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
    let slug = blog.slug;

    // Get By Slug
    let req = test::TestRequest::get()
        .uri(&format!("/app/blogs/slug/{}", slug))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: SuccessResponse<BlogResponseDto> = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.data.unwrap().title, unique_title);
}

#[actix_web::test]
#[serial]
async fn test_get_blog_by_slug_not_found() {
    let container = Container::new();
    seed_user(&container);
    let app = init_test_app!(&container);
    let token = login_admin(&app, &container).await;

    // Get By Slug - Non-existent
    let req = test::TestRequest::get()
        .uri("/app/blogs/slug/non-existent-slug-12345")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
}
