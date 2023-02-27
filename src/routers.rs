use actix_web::{HttpResponse, error::Error, web::Query, get};
use crate::db::establish_connection;
use crate::models::GetPostList;
use crate::cruds::{get_posts};

#[get("/posts")]
pub async fn get_post_list(Query(get_post_list): Query<GetPostList>) -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    let posts = get_posts(conn, get_post_list.length, get_post_list.pages).unwrap();
    Ok(HttpResponse::Ok().json(posts))
}
