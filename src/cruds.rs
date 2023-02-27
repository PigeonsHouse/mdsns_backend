use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::schema::users;

fn get_list_users(conn: &mut PgConnection) -> Vec<crate::models::User> {
    users::dsl::users.select((users::description ,users::id, users::name, users::created_at, users::updated_at)).load(conn).expect("Error getting new user")
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
