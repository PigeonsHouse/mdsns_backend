use actix_web::{App, HttpServer, web, http};
use actix_web_lab::middleware::from_fn;
use actix_web::middleware::Logger;
use dotenvy::dotenv;
use env_logger::Env;
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
    let cors = Cors::default()
        .allowed_origin("*");
    HttpServer::new(|| {
        App::new()
            .wrap(cors)
            .wrap(from_fn(middle::middle_auth))
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(routers::get_post_list)
                    .service(routers::get_post_info)
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
