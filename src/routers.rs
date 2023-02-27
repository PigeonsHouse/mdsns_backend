use actix_web::{HttpResponse, error::Error, web::{Query, Path}, get};
use crate::db::establish_connection;
use crate::models::GetPostList;
use crate::cruds::{get_posts, get_post_info_by_id, GetPostErr};

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

#[get("/posts/{post_id}")]
pub async fn get_post_info(post_id: Path<String>) -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    let post = match get_post_info_by_id(conn, post_id.into_inner()) {
        Ok(post) => post,
        Err(e) => return match e {
            GetPostErr::InvalidParam => Ok(HttpResponse::BadRequest().body("post_id is invalid")),
            GetPostErr::NotFound => Ok(HttpResponse::NotFound().body("post not found")),
            GetPostErr::InternalServerError => Ok(HttpResponse::InternalServerError().body("internal server error"))
        }
    };
    Ok(HttpResponse::Ok().json(post))
}
