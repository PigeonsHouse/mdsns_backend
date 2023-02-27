use crate::models::{GetPostList, PostPost};
use crate::middle::{middle_get_user_id, CheckFirebaseErr};
use actix_web::{HttpResponse, HttpRequest, error::Error, Responder, web::{Query, Path, Json}, get, post, delete};
use crate::db::establish_connection;
use crate::cruds::{get_posts, get_post_info_by_id, GetPostErr, create_new_post, add_favorite, FavoriteErr, remove_favorite};

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

#[post("/favorites/{post_id}")]
pub async fn post_favorite(post_id: Path<String>) -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    let user_id = String::from("hoge_id");
    let post_info = match add_favorite(conn, user_id, post_id.into_inner()) {
        Ok(p) => p,
        Err(e) => return match e {
            FavoriteErr::InvalidParam => Ok(HttpResponse::BadRequest().body("post_id is invalid")),
            FavoriteErr::NotFound => Ok(HttpResponse::NotFound().body("post_id is not found")),
            FavoriteErr::InternalServerError => Ok(HttpResponse::InternalServerError().body("internal server error"))
        }
    };
    Ok(HttpResponse::Ok().json(post_info))
}

#[delete("/favorites/{post_id}")]
pub async fn delete_favorite(post_id: Path<String>) -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    let user_id = String::from("hoge_id");
    let post_info = match remove_favorite(conn, user_id, post_id.into_inner()) {
        Ok(p) => p,
        Err(e) => return match e {
            FavoriteErr::InvalidParam => Ok(HttpResponse::BadRequest().body("post_id is invalid")),
            FavoriteErr::NotFound => Ok(HttpResponse::NotFound().body("post_id is not found")),
            FavoriteErr::InternalServerError => Ok(HttpResponse::InternalServerError().body("internal server error"))
        }
    };
    Ok(HttpResponse::Ok().json(post_info))
}
