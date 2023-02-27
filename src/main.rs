use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::{App, HttpServer, HttpResponse};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_web_lab::middleware::{Next, from_fn};
use dotenvy::dotenv;
use env_logger::Env;
use fireauth::api::RefreshIdToken;
use futures::FutureExt;
use log::{info, debug, error};
use mdsns_backend::auth::{MinimalAuthResult, MinimalAuthErr};
use mdsns_backend::routers;
mod auth;
mod db;
mod cruds;
mod models;
mod schema;

async fn middle_auth(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    // pre-processing
    debug!("{:?}", req);
    let local_id = auth::minimal_auth(req.request()).await;
    next.call(req).await
    // post-processing
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    HttpServer::new(|| {
        App::new()
            .wrap(from_fn(middle_auth))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
