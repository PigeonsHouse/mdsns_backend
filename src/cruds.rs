use log::error;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::insert_into;
use chrono::NaiveDateTime;
//use crate::schema::favorites::created_at;
use crate::schema::users;
use crate::models::{User, NewUser};

fn get_list_users(conn: &mut PgConnection) -> Vec<crate::models::User> {
    users::dsl::users.select((users::description ,
                              users::id,
                              users::name,
                              users::email,
                              users::created_at,
                              users::updated_at
                            )).load(conn).expect("Error getting new user")
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
    users::dsl::users.select((
            users::description,
            users::id,
            users::name,
            users::email,
            users::created_at,
            users::updated_at
        )).first(conn).expect("Error getting new user")
}
