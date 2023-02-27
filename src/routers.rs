use actix_web::{HttpResponse, error::Error, web::{Query, Json}};
use crate::db::establish_connection;
use crate::models::GetPostList;
use crate::cruds::{get_posts};

pub fn get_post_list(pagination_info: Query<GetPostList>) -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    let posts = get_posts(conn, pagination_info.length, pagination_info.pages).unwrap();
    Ok(HttpResponse::Ok().json(posts))
}
