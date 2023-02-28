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
use crate::models::{User, NewUser, Post, PostInfo, NewPost, NewFavorite, NewReply, Favorite};

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
    // https://docs.rs/diesel/latest/diesel/associations/index.html
    // https://diesel.rs/guides/relations.html#reading-data
    // https://diesel.rs/news/2_0_0_release.html#support-for-group-by-clauses
    //
    // let post_data = posts::table.inner_join(users::table).left_join(favorites::table)
    //     .group_by(posts::id).group_by(users::id)
    //     .select((Post::as_select(), User::as_select(), count(favorites::post_id)))
    //     .filter(posts::reply_at.is_null())
    //     .limit(i64::from(length))
    //     .offset(i64::from(pages * length))
    //     .load::<(Post, User, i64)>(conn).unwrap();
    // let reply = diesel::alias!(posts as reply);
    // let post_data = posts::table.inner_join(users::table)
    //     .left_join(favorites::table.on(posts::id).eq(favorites::post_id))
    //     .left_join(reply.on(posts::id).eq(reply.field(posts::reply_at)))
    //     .group_by(posts::id).group_by(users::id)
    //     .limit(i64::from(length))
    //     .offset(i64::from(pages * length))
    //     .select(( Post::as_select(), User::as_select(), count(favorites::post_id), count(reply.field(posts::id)) ))
    //     .order(posts::created_at.desc())
    //     .load::<(Post, User, i64, i64)>(conn).unwrap();

    // let all_users = users::table.load::<User>(conn).unwrap();
    // let all_posts = Post::belonging_to(&all_users)
    //     .limit(i64::from(length))
    //     .offset(i64::from(pages * length))
    //     .load::<Post>(conn).unwrap()
    //     .grouped_by(&all_users);
    // let data = all_users.into_iter().zip(all_posts).collect::<Vec<_>>();
    //
    // for datum in data {
    //     datum
    // }

    let mut post_info = vec![];
    let all_posts = posts::table
        .select(Post::as_select()).limit(i64::from(length)).offset(i64::from(pages * length))
        .filter(posts::reply_at.is_null()).load::<Post>(conn).unwrap();
    for post in all_posts {
        let author = users::table.select(User::as_select()).filter(users::id.eq(&post.user_id)).first::<User>(conn).unwrap();
        let favorited_list = favorites::table.select(Favorite::as_select()).filter(favorites::post_id.eq(&post.id)).load::<Favorite>(conn).unwrap();
        let mut favorited_user_list = vec![];
        for favorited in favorited_list {
            favorited_user_list.push(users::table.select(User::as_select()).filter(users::id.eq(&favorited.user_id)).first::<User>(conn).unwrap());
        }
        let reply_id_list = posts::table.select(Post::as_select()).filter(posts::reply_at.eq(&post.id)).load::<Post>(conn).unwrap();
        post_info.push(PostInfo {
            base_post: post.clone(),
            author,
            favorite_count: favorited_user_list.clone().len() as i64,
            favorited_by: favorited_user_list.clone(),
            replied_count: reply_id_list.clone().len() as i64
        })
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
    let post: Post = posts::table.select(Post::as_select()).filter(posts::id.eq(post_uuid)).first::<Post>(conn).unwrap();

    let author = users::table.select(User::as_select()).filter(users::id.eq(&post.user_id)).first::<User>(conn).unwrap();
    let favorited_list = favorites::table.select(Favorite::as_select()).filter(favorites::post_id.eq(&post.id)).load::<Favorite>(conn).unwrap();
    let mut favorited_user_list = vec![];
    for favorited in favorited_list {
        favorited_user_list.push(users::table.select(User::as_select()).filter(users::id.eq(&favorited.user_id)).first::<User>(conn).unwrap());
    }
    let reply_id_list = posts::table.select(Post::as_select()).filter(posts::reply_at.eq(&post.id)).load::<Post>(conn).unwrap();
    Ok(PostInfo {
        base_post: post.clone(),
        author,
        favorite_count: favorited_user_list.clone().len() as i64,
        favorited_by: favorited_user_list.clone(),
        replied_count: reply_id_list.clone().len() as i64
    })
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
                    Err(_) => return Err(FavoriteErr::InternalServerError)
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
    match posts::table.find(&post_uuid).select(Post::as_select()).load::<Post>(conn) {
        Ok(_) => (),
        Err(e) => return match e {
            Error::NotFound => Err(FavoriteErr::NotFound),
            _ => Err(FavoriteErr::InternalServerError)
        }
    }
    // favoriteをすでに削除していないか検証
    match favorites::table.find((&user_id, &post_uuid)).select(Favorite::as_select()).load::<Favorite>(conn) {
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
