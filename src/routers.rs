use actix_web::{get, post, HttpResponse, Responder, web::Json, error::Error};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
