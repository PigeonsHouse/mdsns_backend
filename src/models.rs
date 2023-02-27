use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use diesel::prelude::{Queryable, Identifiable, Associations};
use std::cmp::PartialEq;
use uuid::Uuid;
use crate::schema::{users, posts};

#[derive(Serialize, Identifiable, Queryable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub description: Option<String>,
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Deserialize)]
pub struct GetPostList {
    pub length: i32,
    pub pages: i32
}

#[derive(Queryable, Serialize, Identifiable, Associations, PartialEq)]
#[diesel(table_name = posts)]
#[diesel(belongs_to(User, foreign_key = author_id))]
pub struct Post {
    pub id: Uuid,
    pub content_md: String,
    pub content_html: String,
    #[diesel(column_name = author_id)]
    pub user_id: String,
    pub reply_at: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize)]
pub struct PostInfo {
    base_post: Post,
    author: User,
    favorited_by: Vec<User>,
    pub favorite_count: i64,
    pub replied_count: i64,
}
