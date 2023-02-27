use crate::db::establish_connection;
use crate::models::GetPostList;
use crate::cruds::get_posts;
use actix_web::{get, post, HttpResponse, Responder, web::{Json, Query}, error::Error, body};

#[get("/posts")]
pub async fn get_post_list(Query(get_post_list): Query<GetPostList>) -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    let posts = get_posts(conn, get_post_list.length, get_post_list.pages).unwrap();
    Ok(HttpResponse::Ok().json(posts))
}

#[post("/posts")]
pub async fn post_md(post: web::Json<>) -> impl Responder {
    HttpResponse::Ok().body("hoge")
}
