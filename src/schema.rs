// @generated automatically by Diesel CLI.

diesel::table! {
    favorites (user_id, post_id) {
        user_id -> Varchar,
        post_id -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        author_id -> Varchar,
        content_md -> Varchar,
        content_html -> Varchar,
        reply_at -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        email -> Varchar,
        description -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(favorites -> posts (post_id));
diesel::joinable!(favorites -> users (user_id));
diesel::joinable!(posts -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    favorites,
    posts,
    users,
);
