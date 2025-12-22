use crate::utils::di::Container;
use crate::utils::success_response::map_success_response;
use actix_web::{HttpResponse, Responder, get, web};

#[get("/")]
pub async fn index() -> impl Responder {
    let response = map_success_response("Hello, world!".to_string());
    HttpResponse::Ok().json(response)
}

#[utoipa::path(
    path = "/app/count",
    tag = "Home",
    responses(
        (status = 200, description = "Count incremented", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>)
    )
)]
#[get("/count")]
pub async fn count(container: web::Data<Container>) -> impl Responder {
    let result = container
        .count_usecase
        .increment()
        .expect("Failed to increment count");
    let response = map_success_response(result.to_string());
    HttpResponse::Ok().json(response)
}

#[utoipa::path(
    path = "/app/send-email",
    tag = "Home",
    responses(
        (status = 200, description = "Email sent", body = crate::utils::success_response::SuccessResponse<crate::utils::success_response::Empty>)
    )
)]
#[get("/send-email")]
pub async fn send_email(container: web::Data<Container>) -> impl Responder {
    let result = container.send_email_usecase.send();
    match result {
        Ok(message) => {
            let response = map_success_response(message);
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}
