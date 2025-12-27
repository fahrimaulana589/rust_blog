use crate::app::features::projects::interface::dto::{
    CreateProjectRequestDto, CreateStackRequestDto, PaginationRequestDto, ProjectResponseDto,
    StackResponseDto, UpdateProjectRequestDto, UpdateStackRequestDto,
};
use crate::utils::di::Container;
use crate::utils::{
    error_response::{ErrorResponse, map_validation_error},
    success_response::SuccessResponse,
};
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use validator::Validate;

// --- Projects ---

#[utoipa::path(
    post,
    path = "/app/projects",
    tag = "Projects",
    request_body = CreateProjectRequestDto,
    responses(
        (status = 201, description = "Project created", body = crate::utils::success_response::SuccessResponse<ProjectResponseDto>),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/projects")]
pub async fn create_project(
    data: web::Data<Container>,
    payload: web::Json<CreateProjectRequestDto>,
) -> impl Responder {
    use crate::app::features::projects::domain::error::ProjectError;

    match data.create_project_usecase.execute(payload.into_inner()) {
        Ok(res) => HttpResponse::Created().json(SuccessResponse::new(
            "Project created successfully".to_string(),
            Some(res),
        )),
        Err(e) => match e {
            ProjectError::Validation(e) => HttpResponse::BadRequest().json(map_validation_error(e)),
            ProjectError::NotFound(msg) => {
                HttpResponse::NotFound().json(crate::utils::error_response::map_string_error(msg))
            }
            ProjectError::System(msg) => HttpResponse::InternalServerError()
                .json(crate::utils::error_response::map_string_error(msg)),
        },
    }
}

#[utoipa::path(
    path = "/app/projects",
    tag = "Projects",
    params(
        PaginationRequestDto
    ),
    responses(
        (status = 200, description = "List projects", body = crate::utils::success_response::SuccessResponse<crate::app::features::projects::interface::dto::PaginatedResponseDto<ProjectResponseDto>>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/projects")]
pub async fn get_all_projects(
    data: web::Data<Container>,
    query: web::Query<PaginationRequestDto>,
) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);

    match data.get_all_projects_usecase.execute(page, per_page) {
        Ok(res) => HttpResponse::Ok().json(SuccessResponse::new(
            "Projects retrieved successfully".to_string(),
            Some(res),
        )),
        Err(e) => HttpResponse::InternalServerError()
            .json(crate::utils::error_response::map_string_error(e)),
    }
}

#[utoipa::path(
    path = "/app/projects/{id}",
    tag = "Projects",
    params(
        ("id", description = "Project ID")
    ),
    responses(
        (status = 200, description = "Project found", body = crate::utils::success_response::SuccessResponse<ProjectResponseDto>),
        (status = 404, description = "Project not found", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/projects/{id}")]
pub async fn get_project(data: web::Data<Container>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match data.get_project_usecase.execute(id) {
        Ok(res) => HttpResponse::Ok().json(SuccessResponse::new(
            "Project retrieved successfully".to_string(),
            Some(res),
        )),
        Err(e) => {
            if e.contains("not found") {
                HttpResponse::NotFound().json(crate::utils::error_response::map_string_error(e))
            } else {
                HttpResponse::InternalServerError()
                    .json(crate::utils::error_response::map_string_error(e))
            }
        }
    }
}

#[utoipa::path(
    put,
    path = "/app/projects/{id}",
    tag = "Projects",
    params(
        ("id", description = "Project ID")
    ),
    request_body = UpdateProjectRequestDto,
    responses(
        (status = 200, description = "Project updated", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 404, description = "Project not found", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/projects/{id}")]
pub async fn update_project(
    data: web::Data<Container>,
    path: web::Path<i32>,
    payload: web::Json<UpdateProjectRequestDto>,
) -> impl Responder {
    use crate::app::features::projects::domain::error::ProjectError;

    let id = path.into_inner();

    match data
        .update_project_usecase
        .execute(id, payload.into_inner())
    {
        Ok(res) => HttpResponse::Ok().json(SuccessResponse::new(
            "Project updated successfully".to_string(),
            Some(res),
        )),
        Err(e) => match e {
            ProjectError::Validation(e) => HttpResponse::BadRequest().json(map_validation_error(e)),
            ProjectError::NotFound(msg) => {
                HttpResponse::NotFound().json(crate::utils::error_response::map_string_error(msg))
            }
            ProjectError::System(msg) => HttpResponse::InternalServerError()
                .json(crate::utils::error_response::map_string_error(msg)),
        },
    }
}

#[utoipa::path(
    path = "/app/projects/{id}",
    tag = "Projects",
    params(
        ("id", description = "Project ID")
    ),
    responses(
        (status = 200, description = "Project deleted", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>),
        (status = 404, description = "Project not found", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/projects/{id}")]
pub async fn delete_project(data: web::Data<Container>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match data.delete_project_usecase.execute(id) {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse::<()>::new(
            "Project deleted successfully".to_string(),
            None,
        )),
        Err(e) => {
            if e.contains("not found") {
                HttpResponse::NotFound().json(crate::utils::error_response::map_string_error(e))
            } else {
                HttpResponse::InternalServerError()
                    .json(crate::utils::error_response::map_string_error(e))
            }
        }
    }
}

// --- Stacks ---

#[utoipa::path(
    path = "/app/stacks",
    tag = "Stacks",
    request_body = CreateStackRequestDto,
    responses(
        (status = 201, description = "Stack created", body = crate::utils::success_response::SuccessResponse<StackResponseDto>),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/stacks")]
pub async fn create_stack(
    data: web::Data<Container>,
    payload: web::Json<CreateStackRequestDto>,
) -> impl Responder {
    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(map_validation_error(e));
    }

    match data.create_stack_usecase.execute(payload.into_inner()) {
        Ok(res) => HttpResponse::Created().json(SuccessResponse::new(
            "Stack created successfully".to_string(),
            Some(res),
        )),
        Err(e) => HttpResponse::InternalServerError()
            .json(crate::utils::error_response::map_string_error(e)),
    }
}

#[utoipa::path(
    path = "/app/stacks",
    tag = "Stacks",
    params(
        PaginationRequestDto
    ),
    responses(
        (status = 200, description = "List stacks", body = crate::utils::success_response::SuccessResponse<crate::app::features::projects::interface::dto::PaginatedResponseDto<StackResponseDto>>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/stacks")]
pub async fn get_all_stacks(
    data: web::Data<Container>,
    query: web::Query<PaginationRequestDto>,
) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);

    match data.get_all_stacks_usecase.execute(page, per_page) {
        Ok(res) => HttpResponse::Ok().json(SuccessResponse::new(
            "Stacks retrieved successfully".to_string(),
            Some(res),
        )),
        Err(e) => HttpResponse::InternalServerError()
            .json(crate::utils::error_response::map_string_error(e)),
    }
}

#[utoipa::path(
    path = "/app/stacks/{id}",
    tag = "Stacks",
    params(
        ("id", description = "Stack ID")
    ),
    responses(
        (status = 200, description = "Stack found", body = crate::utils::success_response::SuccessResponse<StackResponseDto>),
        (status = 404, description = "Stack not found", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/stacks/{id}")]
pub async fn get_stack(data: web::Data<Container>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match data.get_stack_usecase.execute(id) {
        Ok(res) => HttpResponse::Ok().json(SuccessResponse::new(
            "Stack retrieved successfully".to_string(),
            Some(res),
        )),
        Err(e) => {
            if e.contains("not found") {
                HttpResponse::NotFound().json(crate::utils::error_response::map_string_error(e))
            } else {
                HttpResponse::InternalServerError()
                    .json(crate::utils::error_response::map_string_error(e))
            }
        }
    }
}

#[utoipa::path(
    path = "/app/stacks/{id}",
    tag = "Stacks",
    params(
        ("id", description = "Stack ID")
    ),
    request_body = UpdateStackRequestDto, // This is technically from another module but generic name works if imported? Ah, need import or full path? Using `UpdateStackRequestDto` from import
    responses(
        (status = 200, description = "Stack updated", body = crate::utils::success_response::SuccessResponse<StackResponseDto>),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/stacks/{id}")]
pub async fn update_stack(
    data: web::Data<Container>,
    path: web::Path<i32>,
    payload: web::Json<crate::app::features::projects::interface::dto::UpdateStackRequestDto>,
) -> impl Responder {
    let id = path.into_inner();
    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(map_validation_error(e));
    }

    match data.update_stack_usecase.execute(id, payload.into_inner()) {
        Ok(res) => HttpResponse::Ok().json(SuccessResponse::new(
            "Stack updated successfully".to_string(),
            Some(res),
        )),
        Err(e) => {
            if e.contains("not found") {
                HttpResponse::NotFound().json(crate::utils::error_response::map_string_error(e))
            } else {
                HttpResponse::InternalServerError()
                    .json(crate::utils::error_response::map_string_error(e))
            }
        }
    }
}

#[utoipa::path(
    path = "/app/stacks/{id}",
    tag = "Stacks",
    params(
        ("id", description = "Stack ID")
    ),
    responses(
        (status = 200, description = "Stack deleted", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>),
        (status = 404, description = "Stack not found", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/stacks/{id}")]
pub async fn delete_stack(data: web::Data<Container>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match data.delete_stack_usecase.execute(id) {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse::<()>::new(
            "Stack deleted successfully".to_string(),
            None,
        )),
        Err(e) => {
            if e.contains("not found") {
                HttpResponse::NotFound().json(crate::utils::error_response::map_string_error(e))
            } else {
                HttpResponse::InternalServerError()
                    .json(crate::utils::error_response::map_string_error(e))
            }
        }
    }
}
