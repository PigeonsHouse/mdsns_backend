use dotenvy::dotenv;
use actix_web::{App, HttpServer};
use mdsns_backend::routers;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
