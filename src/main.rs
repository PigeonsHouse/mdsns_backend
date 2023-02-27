use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::{App, HttpServer, HttpResponse, web};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_web_lab::middleware::{Next, from_fn};
use actix_web::middleware::Logger;
use dotenvy::dotenv;
use env_logger::Env;
use fireauth::api::RefreshIdToken;
use futures::FutureExt;
use log::{info, debug, error};
use mdsns_backend::routers;
mod db;
mod cruds;
mod models;
mod schema;
mod middle;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    HttpServer::new(|| {
        App::new()
            .wrap(from_fn(middle::middle_auth))
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .route("/posts", web::get().to(routers::get_post_list))
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
