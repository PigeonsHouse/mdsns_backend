use serde::Serialize;
use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};
use crate::schema::users;

#[derive(Queryable, Serialize)]
pub struct User {
    pub description: Option<String>,
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: &'a String,
    pub name: &'a String,
    pub email: &'a String,
}

