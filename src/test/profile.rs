use crate::app::features::profile::interface::dto::{
    LanguageDto, ProfileResponseDto, UpsertProfileRequestDto,
};
use crate::init_test_app;
use crate::utils::di::Container;
use crate::utils::success_response::SuccessResponse;
use actix_web::test;
use serial_test::serial;

#[actix_web::test]
#[serial]
async fn test_get_profile_empty() {
    let container = Container::new();
    // No seeding needed for empty
    let app = init_test_app!(&container);

    // Clear profile table to ensure empty (if test db is shared, but serial helps)
    // Ideally we assume empty or clear it.
    // Since we don't have a clear function exposed easily without accessing repo, we assume serial + possibly reset.
    // Or we execute manual diesel delete.
    let pool = crate::utils::db::establish_connection(&container.config.database_url);
    use crate::schema::profiles;
    use diesel::prelude::*;
    let mut conn = pool.get().unwrap();
    diesel::delete(profiles::table).execute(&mut conn).unwrap();

    let token = crate::test::helpers::login_admin(&app, &container).await;
    let req = test::TestRequest::get()
        .uri("/app/profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp: SuccessResponse<Option<ProfileResponseDto>> =
        test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.message, "Profile belum dibuat");
    assert!(resp.data.is_none() || resp.data == Some(None));
    // Check strict null?
    // In deserializing, Some(None) => None usually if Option<Option> handling matches.
    // But struct `SuccessResponse<T>` has `Option<T>`.
    // T = Option<ProfileResponse>.
    // data: Option<Option<ProfileResponse>>.
    // If JSON is {"data": null}, serde deserializes to Some(None).
    // If JSON is {}, serde deserializes to None (if default) or error.
    // Our SuccessResponse has skip_serializing_if.
}

#[actix_web::test]
#[serial]
async fn test_upsert_profile() {
    let container = Container::new();
    let app = init_test_app!(&container);

    let pool = crate::utils::db::establish_connection(&container.config.database_url);
    use crate::schema::profiles;
    use diesel::prelude::*;
    let mut conn = pool.get().unwrap();
    diesel::delete(profiles::table).execute(&mut conn).unwrap();
    let token = crate::test::helpers::login_admin(&app, &container).await;

    let upsert_dto = UpsertProfileRequestDto {
        full_name: "John Doe".to_string(),
        headline: "Software Engineer".to_string(),
        summary: "Passionate coder".to_string(),
        role: "Developer".to_string(),
        location: "New York".to_string(),
        profile_image: "url".to_string(),
        availability: "Full-time".to_string(),
        years_of_experience: 5,
        resume_url: "resume_url".to_string(),
        email: "john@example.com".to_string(),
        work_philosophy: "Agile".to_string(),
        timezone: "EST".to_string(),
        specializations: vec!["Rust".to_string(), "Backend".to_string()],
        tech_focus: vec!["Actix".to_string()],
        languages: vec![LanguageDto {
            name: "English".to_string(),
            level: "Native".to_string(),
        }],
    };

    // 1. Create
    let req = test::TestRequest::post()
        .uri("/app/profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&upsert_dto)
        .to_request();

    let resp: SuccessResponse<ProfileResponseDto> = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.message, "Profile upserted successfully");
    assert_eq!(resp.data.as_ref().unwrap().full_name, "John Doe");
    assert_eq!(resp.data.as_ref().unwrap().specializations.len(), 2);

    // 2. Update
    let mut update_dto = upsert_dto.clone();
    update_dto.full_name = "Jane Doe".to_string();
    update_dto.specializations = vec!["Go".to_string()]; // Replace

    let req = test::TestRequest::post()
        .uri("/app/profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&update_dto)
        .to_request();

    let resp: SuccessResponse<ProfileResponseDto> = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.data.as_ref().unwrap().full_name, "Jane Doe");
    assert_eq!(resp.data.as_ref().unwrap().specializations.len(), 1);
    assert_eq!(resp.data.as_ref().unwrap().specializations[0], "Go");
}
