use crate::app::features::portofolio::interface::dto::{
    CreatePortofolioRequestDto, PaginationRequestDto, PortofolioResponseDto,
    UpdatePortofolioRequestDto,
};
use crate::utils::di::Container;
use crate::utils::error_response::{ErrorResponse, map_string_error, map_validation_error};
use crate::utils::success_response::{map_success_response, map_success_with_data};
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use validator::Validate;

#[utoipa::path(
    path = "/app/portofolios",
    tag = "Portofolios",
    request_body = CreatePortofolioRequestDto,
    responses(
        (status = 201, description = "Portofolio created", body = crate::utils::success_response::SuccessResponse<PortofolioResponseDto>),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/portofolios")]
pub async fn create_portfolio(
    data: web::Data<Container>,
    payload: web::Json<CreatePortofolioRequestDto>,
) -> impl Responder {
    use crate::app::features::portofolio::domain::error::PortofolioError;

    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(map_validation_error(e));
    }

    match data.portofolio_create_usecase.execute(payload.into_inner()) {
        Ok(res) => HttpResponse::Created()
            .json(map_success_with_data("Portofolio created".to_string(), res)),
        Err(e) => match e {
            PortofolioError::Validation(e) => {
                HttpResponse::BadRequest().json(map_validation_error(e))
            }
            PortofolioError::NotFound(msg) => HttpResponse::NotFound().json(map_string_error(msg)),
            PortofolioError::System(msg) => {
                HttpResponse::InternalServerError().json(map_string_error(msg))
            }
        },
    }
}

#[utoipa::path(
    path = "/app/portofolios",
    tag = "Portofolios",
    responses(
        (status = 200, description = "List portofolios", body = crate::utils::success_response::SuccessResponse<crate::app::features::portofolio::interface::dto::PaginatedResponseDto<PortofolioResponseDto>>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/portofolios")]
pub async fn get_all_portfolios(
    data: web::Data<Container>,
    query: web::Query<PaginationRequestDto>,
) -> impl Responder {
    use crate::app::features::portofolio::domain::error::PortofolioError;
    match data.portofolio_get_all_usecase.execute(query.into_inner()) {
        Ok(res) => {
            HttpResponse::Ok().json(map_success_with_data("List portofolios".to_string(), res))
        }
        Err(e) => match e {
            PortofolioError::Validation(e) => {
                HttpResponse::BadRequest().json(map_validation_error(e))
            }
            PortofolioError::NotFound(msg) => HttpResponse::NotFound().json(map_string_error(msg)),
            PortofolioError::System(msg) => {
                HttpResponse::InternalServerError().json(map_string_error(msg))
            }
        },
    }
}

#[utoipa::path(
    path = "/app/portofolios/{id}",
    tag = "Portofolios",
    params(
        ("id", description = "Portofolio ID")
    ),
    responses(
        (status = 200, description = "Portofolio found", body = crate::utils::success_response::SuccessResponse<PortofolioResponseDto>),
        (status = 404, description = "Portofolio not found", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/portofolios/{id}")]
pub async fn get_portfolio(data: web::Data<Container>, id: web::Path<i32>) -> impl Responder {
    use crate::app::features::portofolio::domain::error::PortofolioError;
    match data.portofolio_get_usecase.execute(id.into_inner()) {
        Ok(res) => {
            HttpResponse::Ok().json(map_success_with_data("Portofolio found".to_string(), res))
        }
        Err(e) => match e {
            PortofolioError::Validation(e) => {
                HttpResponse::BadRequest().json(map_validation_error(e))
            }
            PortofolioError::NotFound(msg) => HttpResponse::NotFound().json(map_string_error(msg)),
            PortofolioError::System(msg) => {
                HttpResponse::InternalServerError().json(map_string_error(msg))
            }
        },
    }
}

#[utoipa::path(
    path = "/app/portofolios/{id}",
    tag = "Portofolios",
    params(
        ("id", description = "Portofolio ID")
    ),
    request_body = UpdatePortofolioRequestDto,
    responses(
        (status = 200, description = "Portofolio updated", body = crate::utils::success_response::SuccessResponse<PortofolioResponseDto>),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/portofolios/{id}")]
pub async fn update_portfolio(
    data: web::Data<Container>,
    id: web::Path<i32>,
    payload: web::Json<UpdatePortofolioRequestDto>,
) -> impl Responder {
    use crate::app::features::portofolio::domain::error::PortofolioError;

    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(map_validation_error(e));
    }

    match data
        .portofolio_update_usecase
        .execute(id.into_inner(), payload.into_inner())
    {
        Ok(res) => {
            HttpResponse::Ok().json(map_success_with_data("Portofolio updated".to_string(), res))
        }
        Err(e) => match e {
            PortofolioError::Validation(e) => {
                HttpResponse::BadRequest().json(map_validation_error(e))
            }
            PortofolioError::NotFound(msg) => HttpResponse::NotFound().json(map_string_error(msg)),
            PortofolioError::System(msg) => {
                HttpResponse::InternalServerError().json(map_string_error(msg))
            }
        },
    }
}

#[utoipa::path(
    path = "/app/portofolios/{id}",
    tag = "Portofolios",
    params(
        ("id", description = "Portofolio ID")
    ),
    responses(
        (status = 200, description = "Portofolio deleted", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>),
        (status = 404, description = "Portofolio not found", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/portofolios/{id}")]
pub async fn delete_portfolio(data: web::Data<Container>, id: web::Path<i32>) -> impl Responder {
    use crate::app::features::portofolio::domain::error::PortofolioError;
    match data.portofolio_delete_usecase.execute(id.into_inner()) {
        Ok(_) => HttpResponse::Ok().json(map_success_response("Portofolio deleted".to_string())),
        Err(e) => match e {
            PortofolioError::Validation(e) => {
                HttpResponse::BadRequest().json(map_validation_error(e))
            }
            PortofolioError::NotFound(msg) => HttpResponse::NotFound().json(map_string_error(msg)),
            PortofolioError::System(msg) => {
                HttpResponse::InternalServerError().json(map_string_error(msg))
            }
        },
    }
}
