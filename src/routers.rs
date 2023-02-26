use actix_web::{HttpResponse, error::Error, get};
use crate::models::HelloMessage;

#[get("/")]
pub async fn hello() -> Result<HttpResponse, Error> {
    let message = HelloMessage { message: String::from("hello, actix-web!") };
    Ok(HttpResponse::Ok().json(message))
}
