use crate::app::features::portfolio::interface::dto::{
    CreatePortfolioRequestDto, PaginationRequestDto, PortfolioResponseDto,
    UpdatePortfolioRequestDto,
};
use crate::utils::di::Container;
use crate::utils::error_response::{ErrorResponse, map_string_error, map_validation_error};
use crate::utils::success_response::{map_success_response, map_success_with_data};
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use validator::Validate;

#[utoipa::path(
    path = "/app/portfolios",
    tag = "Portfolios",
    request_body = CreatePortfolioRequestDto,
    responses(
        (status = 201, description = "Portfolio created", body = crate::utils::success_response::SuccessResponse<PortfolioResponseDto>),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/portfolios")]
pub async fn create_portfolio(
    data: web::Data<Container>,
    payload: web::Json<CreatePortfolioRequestDto>,
) -> impl Responder {
    use crate::app::features::portfolio::domain::error::PortfolioError;

    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(map_validation_error(e));
    }

    match data.portfolio_create_usecase.execute(payload.into_inner()) {
        Ok(res) => HttpResponse::Created()
            .json(map_success_with_data("Portfolio created".to_string(), res)),
        Err(e) => match e {
            PortfolioError::Validation(e) => {
                HttpResponse::BadRequest().json(map_validation_error(e))
            }
            PortfolioError::NotFound(msg) => HttpResponse::NotFound().json(map_string_error(msg)),
            PortfolioError::System(msg) => {
                HttpResponse::InternalServerError().json(map_string_error(msg))
            }
        },
    }
}

#[utoipa::path(
    path = "/app/portfolios",
    tag = "Portfolios",
    responses(
        (status = 200, description = "List portfolios", body = crate::utils::success_response::SuccessResponse<crate::app::features::portfolio::interface::dto::PaginatedResponseDto<PortfolioResponseDto>>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/portfolios")]
pub async fn get_all_portfolios(
    data: web::Data<Container>,
    query: web::Query<PaginationRequestDto>,
) -> impl Responder {
    use crate::app::features::portfolio::domain::error::PortfolioError;
    match data.portfolio_get_all_usecase.execute(query.into_inner()) {
        Ok(res) => {
            HttpResponse::Ok().json(map_success_with_data("List portfolios".to_string(), res))
        }
        Err(e) => match e {
            PortfolioError::Validation(e) => {
                HttpResponse::BadRequest().json(map_validation_error(e))
            }
            PortfolioError::NotFound(msg) => HttpResponse::NotFound().json(map_string_error(msg)),
            PortfolioError::System(msg) => {
                HttpResponse::InternalServerError().json(map_string_error(msg))
            }
        },
    }
}

#[utoipa::path(
    path = "/app/portfolios/{id}",
    tag = "Portfolios",
    params(
        ("id", description = "Portfolio ID")
    ),
    responses(
        (status = 200, description = "Portfolio found", body = crate::utils::success_response::SuccessResponse<PortfolioResponseDto>),
        (status = 404, description = "Portfolio not found", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/portfolios/{id}")]
pub async fn get_portfolio(data: web::Data<Container>, id: web::Path<i32>) -> impl Responder {
    use crate::app::features::portfolio::domain::error::PortfolioError;
    match data.portfolio_get_usecase.execute(id.into_inner()) {
        Ok(res) => {
            HttpResponse::Ok().json(map_success_with_data("Portfolio found".to_string(), res))
        }
        Err(e) => match e {
            PortfolioError::Validation(e) => {
                HttpResponse::BadRequest().json(map_validation_error(e))
            }
            PortfolioError::NotFound(msg) => HttpResponse::NotFound().json(map_string_error(msg)),
            PortfolioError::System(msg) => {
                HttpResponse::InternalServerError().json(map_string_error(msg))
            }
        },
    }
}

#[utoipa::path(
    path = "/app/portfolios/{id}",
    tag = "Portfolios",
    params(
        ("id", description = "Portfolio ID")
    ),
    request_body = UpdatePortfolioRequestDto,
    responses(
        (status = 200, description = "Portfolio updated", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/portfolios/{id}")]
pub async fn update_portfolio(
    data: web::Data<Container>,
    id: web::Path<i32>,
    payload: web::Json<UpdatePortfolioRequestDto>,
) -> impl Responder {
    use crate::app::features::portfolio::domain::error::PortfolioError;

    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(map_validation_error(e));
    }

    match data
        .portfolio_update_usecase
        .execute(id.into_inner(), payload.into_inner())
    {
        Ok(_) => HttpResponse::Ok().json(map_success_response("Portfolio updated".to_string())),
        Err(e) => match e {
            PortfolioError::Validation(e) => {
                HttpResponse::BadRequest().json(map_validation_error(e))
            }
            PortfolioError::NotFound(msg) => HttpResponse::NotFound().json(map_string_error(msg)),
            PortfolioError::System(msg) => {
                HttpResponse::InternalServerError().json(map_string_error(msg))
            }
        },
    }
}

#[utoipa::path(
    path = "/app/portfolios/{id}",
    tag = "Portfolios",
    params(
        ("id", description = "Portfolio ID")
    ),
    responses(
        (status = 200, description = "Portfolio deleted", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>),
        (status = 404, description = "Portfolio not found", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/portfolios/{id}")]
pub async fn delete_portfolio(data: web::Data<Container>, id: web::Path<i32>) -> impl Responder {
    use crate::app::features::portfolio::domain::error::PortfolioError;
    match data.portfolio_delete_usecase.execute(id.into_inner()) {
        Ok(_) => HttpResponse::Ok().json(map_success_response("Portfolio deleted".to_string())),
        Err(e) => match e {
            PortfolioError::Validation(e) => {
                HttpResponse::BadRequest().json(map_validation_error(e))
            }
            PortfolioError::NotFound(msg) => HttpResponse::NotFound().json(map_string_error(msg)),
            PortfolioError::System(msg) => {
                HttpResponse::InternalServerError().json(map_string_error(msg))
            }
        },
    }
}
