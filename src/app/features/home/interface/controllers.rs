use crate::utils::di::Container;
use actix_web::{Responder, get, web};

#[get("/")]
pub async fn index() -> impl Responder {
    "Hello, world!"
}

#[get("/count")]
pub async fn count(container: web::Data<Container>) -> impl Responder {
    let count_val = container
        .count_usecase
        .increment()
        .expect("Error incrementing count");
    format!("Count: {}", count_val)
}
