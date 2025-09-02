// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Uuid,
        title -> Varchar,
        body -> Text,
        created_by -> Uuid,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    posts_tags (fk_post_id, tag) {
        fk_post_id -> Uuid,
        tag -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(posts -> users (created_by));
diesel::joinable!(posts_tags -> posts (fk_post_id));

diesel::allow_tables_to_appear_in_same_query!(posts, posts_tags, users,);
