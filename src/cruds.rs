use core::fmt;
use std::fmt::Debug;
use std::str::FromStr;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::insert_into;
use diesel::result::Error;
use uuid::Uuid;
use crate::schema::{posts, users};
use crate::models::{User, NewUser, Post, PostInfo, PostPost, NewPost};

fn get_list_users(conn: &mut PgConnection) -> Vec<User> {
    users::dsl::users.select(User::as_select()).load::<User>(conn).expect("Error getting new user")
}

pub fn search_user_from_db(conn: &mut PgConnection, id: String) -> bool {
    let user_vec = get_list_users(conn);
    match user_vec
        .iter()
        .find(|&user| user.id == id) {
    Some(_) => return true,
    None => return false,
    };
}

pub fn register_user(
    conn: &mut PgConnection,
    id: &String,
    name: &String,
    email: &String
    ) -> User {
    let new_user = NewUser{id, name, email};
    insert_into(users::dsl::users).values(&new_user)
        .execute(conn)
        .expect("Failed to create new user");
    users::dsl::users.select(User::as_select()).first(conn).expect("Error getting new user")
}

pub fn get_posts(conn: &mut PgConnection, length: Option<i32>, pages: Option<i32>) -> Result<Vec<PostInfo>, Error> {
    let length = match length {
        Some(l) => l,
        None => 30
    };
    let pages = match pages {
        Some(p) => p,
        None => 0
    };
    let post_data = posts::table.inner_join(users::table)
        .select((Post::as_select(), User::as_select()))
        .filter(posts::reply_at.is_null())
        .limit(i64::from(length))
        .offset(i64::from(pages * length))
        .load::<(Post, User)>(conn).unwrap();
    let mut post_info = vec![];
    for i in post_data {
        post_info.push(PostInfo { base_post: i.0, author: i.1, favorited_by: vec![], favorite_count: 0, replied_count: 0 })
    }
    Ok(post_info)
}

#[derive(Debug)]
pub enum GetPostErr {
    InvalidParam,
    NotFound,
    InternalServerError
}

pub fn get_post_info_by_id(conn: &mut PgConnection, post_id: String) -> Result<PostInfo, GetPostErr> {
    let post_uuid = match Uuid::from_str(&post_id) {
        Ok(id) => id,
        Err(_) => return Err(GetPostErr::InvalidParam)
    };
    let post_data: (Post, User) = match posts::table.inner_join(users::table)
        .select((Post::as_select(), User::as_select()))
        .filter(posts::id.eq(post_uuid))
        .first::<(Post, User)>(conn) {
        Ok(data) => data,
        Err(e) => return match e {
            Error::NotFound => Err(GetPostErr::NotFound),
            _ => Err(GetPostErr::InternalServerError)
        }
    };

    Ok(PostInfo { base_post: post_data.0, author: post_data.1, favorited_by: vec![], favorite_count: 0, replied_count: 0 })
}

#[derive(Debug)]
pub enum CreatePostErr {
    InvalidParam,
    InternalServerError
}

impl fmt::Display for CreatePostErr {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePostErr::InvalidParam => write!(f, "Paramaters are invalid"),
            CreatePostErr::InternalServerError=> write!(f, "InternalServerError"),
        }
    }
}

pub fn create_new_post (
    conn: &mut PgConnection,
    author_id: &String,
    content_md: &String,
    content_html: &String
    ) -> Result<PostInfo, CreatePostErr> {
    let new_post = NewPost{author_id, content_md, content_html};
    insert_into(posts::dsl::posts).values(&new_post)
        .execute(conn)
        .expect("Failed to create new post");
    let new_post_id: Uuid = posts::dsl::posts.select(posts::id)
        .first(conn)
        .expect("Error getting new post id");
    match get_post_info_by_id(conn, new_post_id.to_string()) {
        Ok(post_info) => Ok(post_info),
        Err(_) => Err(CreatePostErr::InternalServerError),
    }
}

