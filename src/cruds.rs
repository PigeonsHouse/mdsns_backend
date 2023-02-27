use log::error;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::insert_into;
use chrono::NaiveDateTime;
use diesel::result::Error;
use log::info;
use serde_json::json;
use crate::schema::{posts, users};
use crate::models::{User, NewUser, Post, PostInfo};

fn get_list_users(conn: &mut PgConnection) -> Vec<User> {
    users::dsl::users.select(User::as_select()).load::<User>(conn).expect("Error getting new user")
}

pub fn search_user_from_db(conn: &mut PgConnection, id: String) -> bool {
    let user_vec = get_list_users(conn);
    return match user_vec
        .iter()
        .find(|&user| user.id == id) {
        Some(_) => true,
        None => false,
    };
}

pub fn get_posts(conn: &mut PgConnection, length: i32, pages: i32) -> Result<Vec<PostInfo>, Error> {
    // let all_users = users::table.select(User::as_select()).load::<User>(conn).unwrap();
    // let raw_posts = Post::belonging_to(&all_users).load::<Post>(conn).unwrap().grouped_by(&all_users);
    // let get_posts: Vec<(User, Vec<Post>)> = all_users.into_iter().zip(raw_posts).collect();

    let post_data = posts::table.inner_join(users::table).select((Post::as_select(), User::as_select())).load::<(Post, User)>(conn).unwrap();
    let mut post_info = vec![];
    for i in post_data {
        post_info.push(PostInfo { base_post: i.0, author: i.1, favorited_by: vec![], favorite_count: 0, replied_count: 0 })
    }
    info!("{}", json!(post_info));
    Ok(post_info)
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
    users::dsl::users.select((User::as_select())).first(conn).expect("Error getting new user")
}
