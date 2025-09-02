use rocket::serde::json::Json;
use rocket::State;

use crate::db::DbPool;
use crate::models::{NewPostWithTags, NewUser, PaginatedResponse};
use crate::repository::{PostRepository, UserRepository};

#[post("/users", data = "<user_data>")]
pub async fn create_user(
    pool: &State<DbPool>,
    user_data: Json<NewUser>,
) -> Json<serde_json::Value> {
    let new_user = NewUser {
        username: user_data.username.clone(),
        first_name: user_data.first_name.clone(),
        last_name: user_data.last_name.clone(),
    };

    let mut conn = pool.get().expect("Failed to get DB connection from pool.");

    match UserRepository::create(&mut conn, new_user) {
        Ok(user) => Json(serde_json::json!({
            "success": true,
            "data": user
        })),
        Err(_) => Json(serde_json::json!({
            "success": false,
            "error": "Failed to create user"
        })),
    }
}

#[post("/posts", data = "<post_data>")]
pub async fn create_post(
    pool: &State<DbPool>,
    post_data: Json<NewPostWithTags>,
) -> Json<serde_json::Value> {
    let new_post_with_tags = NewPostWithTags {
        title: post_data.title.clone(),
        body: post_data.body.clone(),
        created_by: post_data.created_by,
        tags: post_data.tags.clone(),
    };

    let mut conn = pool.get().expect("Failed to get DB connection from pool.");

    match PostRepository::create_with_tags(&mut conn, new_post_with_tags) {
        Ok(post) => Json(serde_json::json!({
            "success": true,
            "data": post
        })),
        Err(_) => Json(serde_json::json!({
            "success": false,
            "error": "Failed to create post"
        })),
    }
}

#[get("/posts?<page>&<limit>&<search>")]
pub async fn list_posts(
    pool: &State<DbPool>,
    page: Option<i64>,
    limit: Option<i64>,
    search: Option<String>,
) -> Json<serde_json::Value> {
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(10);
    let search = search.as_deref();

    let mut conn = pool.get().expect("Failed to get DB connection from pool.");

    match PostRepository::find_with_user_and_tags(&mut conn, page, limit, search) {
        Ok((posts, meta)) => {
            let response = PaginatedResponse {
                records: posts,
                meta,
            };
            Json(serde_json::json!({
                "success": true,
                "data": response
            }))
        }
        Err(_) => Json(serde_json::json!({
            "success": false,
            "error": "Failed to fetch posts"
        })),
    }
}
