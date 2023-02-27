use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use diesel::prelude::{Queryable, Identifiable, Associations, Selectable, Insertable};
use std::cmp::PartialEq;
use uuid::Uuid;
use crate::schema::{users, posts};

#[derive(Serialize, Identifiable, Queryable, Selectable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(skip)]
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Deserialize)]
pub struct GetPostList {
    pub length: i32,
    pub pages: i32
}

#[derive(Queryable, Serialize, Identifiable, Associations, Selectable, PartialEq)]
#[diesel(table_name = posts)]
#[diesel(belongs_to(User, foreign_key = author_id))]
pub struct Post {
    pub id: Uuid,
    #[diesel(column_name = author_id)]
    #[serde(skip)]
    pub user_id: String,
    pub content_md: String,
    pub content_html: String,
    pub reply_at: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize)]
pub struct PostInfo {
    #[serde(flatten)]
    pub base_post: Post,
    pub author: User,
    pub favorited_by: Vec<User>,
    pub favorite_count: i64,
    pub replied_count: i64,
}
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: &'a String,
    pub name: &'a String,
    pub email: &'a String,
}