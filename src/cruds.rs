use core::fmt;
use std::fmt::Debug;
use std::str::FromStr;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{insert_into, delete};
use diesel::result::Error;
use log::debug;
use uuid::Uuid;
use crate::schema::{posts, users, favorites};
use crate::models::{User, NewUser, Post, PostInfo, PostPost, NewPost, NewFavorite, NewReply};

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
    content_html: &String,
    reply_to: &Option<Uuid>
    ) -> Result<PostInfo, CreatePostErr> {
    let new_post = match reply_to {
        Some(r) => {
            let reply_at = r;
            debug!("reply_at: {:?}", reply_at);
            let new_reply = NewReply{author_id, content_md, content_html, reply_at};
            insert_into(posts::dsl::posts).values(&new_reply)
            .get_result::<Post>(conn)
            .expect("Failed to create new post")
        },
        None => {
            let new_post = NewPost{author_id, content_md, content_html};
            insert_into(posts::dsl::posts).values(&new_post)
            .get_result::<Post>(conn)
            .expect("Failed to create new post")
        },
    };
    match get_post_info_by_id(conn, new_post.id.to_string()) {
        Ok(post_info) => Ok(post_info),
        Err(_) => Err(CreatePostErr::InternalServerError),
    }
}

#[derive(Debug)]
pub enum FavoriteErr {
    InvalidParam,
    NotFound,
    InternalServerError
}

pub fn add_favorite(conn: &mut PgConnection, user_id: String, post_id: String) -> Result<PostInfo, FavoriteErr> {
    // post_idのキャスト兼post_idがUUIDになれるStringか検証
    let post_uuid = match Uuid::from_str(&post_id) {
        Ok(id) => id,
        Err(_) => return Err(FavoriteErr::InvalidParam)
    };
    // post_idがpostsにあるか検証
    match posts::table.find(post_uuid).select(posts::id).get_result::<Uuid>(conn) {
        Ok(_) => (),
        Err(e) => return match e {
            Error::NotFound => Err(FavoriteErr::NotFound),
            _ => Err(FavoriteErr::InternalServerError)
        }
    }
    // favoriteをすでに押していないか検証
    match favorites::table.find((&user_id, post_uuid)).select(favorites::post_id).get_result::<Uuid>(conn) {
        // 押してたらスルー
        Ok(_) => (),
        Err(e) => match e {
            // 押してなかったらデータ追加
            Error::NotFound => {
                let new_favorite = NewFavorite{ user_id: &user_id, post_id: &post_uuid };
                match insert_into(favorites::table).values(&new_favorite).execute(conn) {
                    Ok(_) => (),
                    Err(e) => return Err(FavoriteErr::InternalServerError)
                };
            },
            // エラーはエラー
            _ => return Err(FavoriteErr::InternalServerError)
        }
    }
    // 更新されたpostsを取得してreturn
    return match get_post_info_by_id(conn, post_id) {
        Ok(p) => Ok(p),
        Err(_) => Err(FavoriteErr::InternalServerError)
    }
}

pub fn remove_favorite(conn: &mut PgConnection, user_id: String, post_id: String) -> Result<PostInfo, FavoriteErr> {
    // post_idのキャスト兼post_idがUUIDになれるStringか検証
    let post_uuid = match Uuid::from_str(&post_id) {
        Ok(id) => id,
        Err(_) => return Err(FavoriteErr::InvalidParam)
    };
    // post_idがpostsにあるか検証
    match posts::table.find(&post_uuid).select(posts::id).get_result::<Uuid>(conn) {
        Ok(_) => (),
        Err(e) => return match e {
            Error::NotFound => Err(FavoriteErr::NotFound),
            _ => Err(FavoriteErr::InternalServerError)
        }
    }
    // favoriteをすでに削除していないか検証
    match favorites::table.find((&user_id, &post_uuid)).select(favorites::post_id).get_result::<Uuid>(conn) {
        // 押してたらデータ削除
        Ok(_) => {
            match delete(favorites::dsl::favorites.filter(favorites::post_id.eq(&post_uuid)).filter(favorites::user_id.eq(&user_id))).execute(conn) {
                Ok(_) => (),
                Err(e) => return Err(FavoriteErr::InternalServerError)
            }
        },
        Err(e) => match e {
            // 押してなかったらスルー
            Error::NotFound => (),
            // エラーはエラー
            _ => return Err(FavoriteErr::InternalServerError)
        }
    }
    // 更新されたpostsを取得してreturn
    return match get_post_info_by_id(conn, post_id) {
        Ok(p) => Ok(p),
        Err(_) => Err(FavoriteErr::InternalServerError)
    }
}
