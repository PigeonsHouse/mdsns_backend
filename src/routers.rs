use actix_web::{HttpResponse, Responder, error::Error, web::{Query, Path, Json}, get, post, HttpRequest};
use crate::db::establish_connection;
use crate::models::{GetPostList, PostPost};
use crate::cruds::{get_posts, get_post_info_by_id, GetPostErr, create_new_post};
use crate::middle::{middle_get_user_id, CheckFirebaseErr};

#[get("/posts")]
pub async fn get_post_list(Query(get_post_list): Query<GetPostList>) -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    let posts = get_posts(conn, get_post_list.length, get_post_list.pages).unwrap();
    Ok(HttpResponse::Ok().json(posts))
}

#[post("/posts")]
pub async fn post_md(post_req: HttpRequest, post_json: Json<PostPost>) -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    let auther_id: String = match middle_get_user_id(post_req).await {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::Unauthorized().body("token is missed")),
    };
    match create_new_post(conn, &auther_id, &post_json.content_md, &post_json.content_html) {
        Ok(info) => Ok(HttpResponse::Ok().json(info)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("internal server error")),
    }
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
