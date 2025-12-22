use crate::app::features::projects::interface::dto::{
    CreateProjectRequestDto, CreateStackRequestDto, PaginationRequestDto, UpdateProjectRequestDto,
};
use crate::utils::di::Container;
use crate::utils::{error_response::map_validation_error, success_response::SuccessResponse};
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use validator::Validate;

// --- Projects ---

#[post("/projects")]
pub async fn create_project(
    data: web::Data<Container>,
    payload: web::Json<CreateProjectRequestDto>,
) -> impl Responder {
    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(map_validation_error(e));
    }

    match data.create_project_usecase.execute(payload.into_inner()) {
        Ok(res) => HttpResponse::Created().json(SuccessResponse::new(
            "Project created successfully".to_string(),
            Some(res),
        )),
        Err(e) => HttpResponse::InternalServerError()
            .json(crate::utils::error_response::map_string_error(e)),
    }
}

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

#[put("/projects/{id}")]
pub async fn update_project(
    data: web::Data<Container>,
    path: web::Path<i32>,
    payload: web::Json<UpdateProjectRequestDto>,
) -> impl Responder {
    let id = path.into_inner();
    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(map_validation_error(e));
    }

    match data
        .update_project_usecase
        .execute(id, payload.into_inner())
    {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse::<()>::new(
            "Project updated successfully".to_string(),
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

#[get("/stacks")]
pub async fn get_all_stacks(data: web::Data<Container>) -> impl Responder {
    match data.get_all_stacks_usecase.execute() {
        Ok(res) => HttpResponse::Ok().json(SuccessResponse::new(
            "Stacks retrieved successfully".to_string(),
            Some(res),
        )),
        Err(e) => HttpResponse::InternalServerError()
            .json(crate::utils::error_response::map_string_error(e)),
    }
}

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
        Ok(_) => HttpResponse::Ok().json(SuccessResponse::<()>::new(
            "Stack updated successfully".to_string(),
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
