use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use diesel::prelude::{Queryable, Identifiable, Associations, Selectable, Insertable, QueryableByName};
use std::cmp::PartialEq;
use uuid::Uuid;
use crate::schema::{users, posts, favorites};

#[derive(Serialize, Identifiable, Queryable, Selectable, QueryableByName, PartialEq, Debug, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub name: String,
    #[serde(skip)]
    pub email: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Deserialize)]
pub struct GetPostList {
    pub length: Option<i32>,
    pub pages: Option<i32>
}

#[derive(Deserialize)]
pub struct ReplyTo {
    pub reply_to: Option<Uuid>
}

#[derive(Queryable, Serialize, Identifiable, Associations, Selectable, QueryableByName, PartialEq, Debug, Clone)]
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

#[derive(Deserialize)]
pub struct PostPost {
    pub content_md: String,
    pub content_html: String,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub author_id: &'a String,
    pub content_md: &'a String,
    pub content_html: &'a String,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewReply<'a> {
    pub author_id: &'a String,
    pub content_md: &'a String,
    pub content_html: &'a String,
    pub reply_at: &'a Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: &'a String,
    pub name: &'a String,
    pub email: &'a String,
}

#[derive(Insertable)]
#[diesel(table_name = favorites)]
pub struct NewFavorite<'a> {
    pub user_id: &'a String,
    pub post_id: &'a Uuid
}

#[derive(Queryable, Serialize, Selectable)]
#[diesel(table_name = favorites)]
pub struct Favorite {
    pub user_id: String,
    pub post_id: Uuid,
    pub created_at: NaiveDateTime
}
