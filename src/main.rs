use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use dotenvy::dotenv;
use env_logger::Env;
use mdsns_backend::routers;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
