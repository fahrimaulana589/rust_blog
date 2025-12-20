use crate::utils::di::Container;
use actix_web::{HttpResponse, Responder, get, web};

#[get("/")]
pub async fn index() -> impl Responder {
    "Hello, world!"
}

#[get("/count")]
pub async fn count(container: web::Data<Container>) -> impl Responder {
    let result = container
        .count_usecase
        .increment()
        .expect("Failed to increment count");
    HttpResponse::Ok().json(result)
}

#[get("/send-email")]
pub async fn send_email(container: web::Data<Container>) -> impl Responder {
    let result = container.send_email_usecase.send();
    match result {
        Ok(message) => HttpResponse::Ok().json(message),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}
