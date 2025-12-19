use actix_web::{get, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Hello, world!"
}

#[get("/count")]
async fn count() -> impl Responder {
    "Count"
}