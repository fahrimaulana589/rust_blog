use crate::app::features::profile::interface::dto::{ProfileResponseDto, UpsertProfileRequestDto};
use crate::utils::di::Container;
use crate::utils::error_response::{ErrorResponse, map_string_error, map_validation_error};
use crate::utils::success_response::{SuccessResponse, map_success_with_data};
use actix_web::{HttpResponse, Responder, get, post, web};
use validator::Validate;

#[utoipa::path(
    path = "/app/profile",
    tag = "Profile",
    responses(
        (status = 200, description = "Get profile", body = crate::utils::success_response::SuccessResponse<Option<ProfileResponseDto>>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/profile")]
pub async fn get_profile(container: web::Data<Container>) -> impl Responder {
    match container.get_profile_usecase.execute() {
        Ok(profile_opt) => {
            match profile_opt {
                Some(profile) => HttpResponse::Ok().json(map_success_with_data(
                    "Profile fetched successfully".to_string(),
                    profile,
                )),
                None => HttpResponse::Ok().json(SuccessResponse {
                    message: "Profile belum dibuat".to_string(),
                    data: Some(None::<ProfileResponseDto>), // Explicit null
                }),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(map_string_error(e)),
    }
}

#[utoipa::path(
    path = "/app/profile",
    tag = "Profile",
    request_body = UpsertProfileRequestDto,
    responses(
        (status = 200, description = "Profile upserted", body = crate::utils::success_response::SuccessResponse<ProfileResponseDto>),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/profile")]
pub async fn upsert_profile(
    container: web::Data<Container>,
    payload: web::Json<UpsertProfileRequestDto>,
) -> impl Responder {
    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(map_validation_error(e));
    }
    match container
        .upsert_profile_usecase
        .execute(payload.into_inner())
    {
        Ok(profile) => HttpResponse::Ok().json(map_success_with_data(
            "Profile upserted successfully".to_string(),
            profile,
        )),
        Err(e) => HttpResponse::InternalServerError().json(map_string_error(e)),
    }
}
