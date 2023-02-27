use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use crate::schema::users;
use crate::models::{User, Post};

fn get_list_users(conn: &mut PgConnection) -> Vec<User> {
    users::dsl::users.select((users::id, users::description, users::name, users::created_at, users::updated_at)).load::<User>(conn).expect("Error getting new user")
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

<<<<<<< HEAD
/*
pub fn register_user(conn: &mut PgConnection, id: String) -> bool {
    
}
*/
=======
pub fn get_posts(conn: &mut PgConnection, length: i32, pages: i32) -> Result<Vec<Post>, Error> {
    let users = users::table.select((users::id, users::name, users::created_at, users::updated_at)).load::<User>(conn).unwrap();
    let posts = Post::belonging_to(&users).load::<Post>(conn).unwrap();

    Ok(posts)
}
>>>>>>> 3bd6455 (WIP)
