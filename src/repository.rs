use chrono::{DateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_types::{Array, BigInt, Nullable, Text, Timestamptz, Uuid as SqlUuid};
use uuid::Uuid;

use crate::models::{
    CreatedBy, NewPost, NewPostTag, NewPostWithTags, NewUser, PaginationMeta, Post,
    PostWithUserAndTags, User,
};
use crate::schema::{posts, posts_tags, users};

#[derive(QueryableByName, Debug)]
struct CountResult {
    #[diesel(sql_type = BigInt)]
    count: i64,
}

#[derive(QueryableByName, Debug)]
struct PostWithTagsQueryResult {
    #[diesel(sql_type = SqlUuid)]
    id: Uuid,
    #[diesel(sql_type = Text)]
    title: String,
    #[diesel(sql_type = Text)]
    body: String,
    #[diesel(sql_type = Timestamptz)]
    created_at: DateTime<Utc>,
    #[diesel(sql_type = Nullable<SqlUuid>)]
    user_id: Option<Uuid>,
    #[diesel(sql_type = Nullable<Text>)]
    username: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    first_name: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    last_name: Option<String>,
    #[diesel(sql_type = Array<Nullable<Text>>)]
    tags: Vec<Option<String>>,
}

pub struct UserRepository;

impl UserRepository {
    pub fn create(
        conn: &mut PgConnection,
        new_user: NewUser,
    ) -> Result<User, diesel::result::Error> {
        let user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)?;
        Ok(user)
    }
}

pub struct PostRepository;

impl PostRepository {
    pub fn create_with_tags(
        conn: &mut PgConnection,
        new_post_with_tags: NewPostWithTags,
    ) -> Result<Post, diesel::result::Error> {
        conn.transaction::<Post, diesel::result::Error, _>(|conn| {
            // Create the post first
            let new_post = NewPost {
                title: new_post_with_tags.title,
                body: new_post_with_tags.body,
                created_by: new_post_with_tags.created_by,
            };

            let post = diesel::insert_into(posts::table)
                .values(&new_post)
                .get_result::<Post>(conn)?;

            // Create the tags if any
            if !new_post_with_tags.tags.is_empty() {
                let post_tags: Vec<NewPostTag> = new_post_with_tags
                    .tags
                    .into_iter()
                    .map(|tag| NewPostTag {
                        fk_post_id: post.id,
                        tag,
                    })
                    .collect();

                diesel::insert_into(posts_tags::table)
                    .values(&post_tags)
                    .execute(conn)?;
            }

            Ok(post)
        })
    }

    pub fn find_with_user_and_tags(
        conn: &mut PgConnection,
        page: i64,
        limit: i64,
        search: Option<&str>,
    ) -> Result<(Vec<PostWithUserAndTags>, PaginationMeta), diesel::result::Error> {
        let offset = (page - 1) * limit;

        // Build the count query using Diesel's sql_query with proper bindings
        let count_sql = r#"
            SELECT COUNT(DISTINCT p.id)
            FROM posts p
            LEFT JOIN users u ON p.created_by = u.id
            LEFT JOIN posts_tags pt ON p.id = pt.fk_post_id
            WHERE ($1::text IS NULL OR 
                   p.title ILIKE $1 OR 
                   p.body ILIKE $1 OR 
                   u.username ILIKE $1 OR 
                   u.first_name ILIKE $1 OR 
                   u.last_name ILIKE $1 OR
                   pt.tag ILIKE $1)
        "#;

        let search_pattern = search.map(|s| format!("%{s}%"));
        let count_result: CountResult = diesel::sql_query(count_sql)
            .bind::<Nullable<Text>, _>(search_pattern.as_deref())
            .get_result(conn)?;
        let total_docs = count_result.count;

        let total_pages = (total_docs + limit - 1) / limit;

        // Main query with array aggregation for tags and LEFT JOIN for users
        // This uses Diesel's sql_query but only for the ARRAY_AGG part
        let main_sql = r#"
            SELECT 
                p.id,
                p.title,
                p.body,
                p.created_at,
                u.id as user_id,
                u.username,
                u.first_name,
                u.last_name,
                COALESCE(ARRAY_AGG(DISTINCT pt.tag) FILTER (WHERE pt.tag IS NOT NULL), '{}') as tags
            FROM posts p
            LEFT JOIN users u ON p.created_by = u.id
            LEFT JOIN posts_tags pt ON p.id = pt.fk_post_id
            WHERE ($1::text IS NULL OR 
                   p.title ILIKE $1 OR 
                   p.body ILIKE $1 OR 
                   u.username ILIKE $1 OR 
                   u.first_name ILIKE $1 OR 
                   u.last_name ILIKE $1 OR
                   pt.tag ILIKE $1)
            GROUP BY p.id, p.title, p.body, p.created_at, u.id, u.username, u.first_name, u.last_name
            ORDER BY p.created_at DESC
            LIMIT $2 OFFSET $3
        "#;

        let results: Vec<PostWithTagsQueryResult> = diesel::sql_query(main_sql)
            .bind::<Nullable<Text>, _>(search_pattern.as_deref())
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load(conn)?;

        // Transform results into PostWithUserAndTags structs
        let posts_with_users_and_tags = results
            .into_iter()
            .map(|result| {
                let created_by = if let (Some(user_id), Some(username), Some(first_name)) =
                    (result.user_id, result.username, result.first_name)
                {
                    Some(CreatedBy {
                        user_id,
                        username,
                        first_name,
                        last_name: result.last_name,
                    })
                } else {
                    None
                };

                let tags: Vec<String> = result.tags.into_iter().flatten().collect();

                PostWithUserAndTags {
                    id: result.id,
                    title: result.title,
                    body: result.body,
                    created_by,
                    created_at: result.created_at,
                    tags,
                }
            })
            .collect();

        let meta = PaginationMeta {
            current_page: page,
            per_page: limit,
            from: offset + 1,
            to: std::cmp::min(offset + limit, total_docs),
            total_pages,
            total_docs,
        };

        Ok((posts_with_users_and_tags, meta))
    }
}
