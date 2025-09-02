use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{posts, posts_tags, users};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub created_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPostWithTags {
    pub title: String,
    pub body: String,
    pub created_by: Uuid,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Post, foreign_key = fk_post_id))]
#[diesel(table_name = posts_tags)]
#[diesel(primary_key(fk_post_id, tag))]
pub struct PostTag {
    pub fk_post_id: Uuid,
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = posts_tags)]
pub struct NewPostTag {
    pub fk_post_id: Uuid,
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatedBy {
    pub user_id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostWithUserAndTags {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_by: Option<CreatedBy>,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub records: Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub current_page: i64,
    pub per_page: i64,
    pub from: i64,
    pub to: i64,
    pub total_pages: i64,
    pub total_docs: i64,
}
