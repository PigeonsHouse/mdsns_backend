use serde::Serialize;
use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};

#[derive(Queryable, Serialize)]
pub struct User {
    pub description: Option<String>,
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}
