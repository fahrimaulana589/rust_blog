use crate::utils::di::Container;
use actix_web::{Responder, get, web, HttpResponse};

#[get("/")]
pub async fn index() -> impl Responder {
    "Hello, world!"
}

#[get("/count")]
pub async fn count(container: web::Data<Container>) -> impl Responder {
    let result = container
        .home_usecase
        .increment()
        .expect("Failed to increment count");
    HttpResponse::Ok().json(result)
}
