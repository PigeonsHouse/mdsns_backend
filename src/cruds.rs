use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::ExpressionMethods;

fn get_list_users(conn: &mut PgConnection) -> Vec<crate::models::User> {
    use crate::schema::users;
    use crate::models::User;
    users::dsl::users.select((users::id, users::name, users::created_at, users::updated_at)).load(conn).expect("Error getting new user")
}

pub fn db_sign_in(conn: &mut PgConnection, id: String) -> bool {
    let user_vec = get_list_users(conn);
    match user_vec
        .iter()
        .find(|&user| user.id == id) {
    Some(_) => return true,
    None => return false,
    };
}
