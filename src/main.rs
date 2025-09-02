#[macro_use]
extern crate rocket;

mod db;
mod handlers;
mod models;
mod repository;
mod schema;

use crate::db::establish_connection;
use rocket::fairing::AdHoc;

#[launch]
fn rocket() -> _ {
    let pool = establish_connection();

    rocket::build()
        .manage(pool)
        .attach(AdHoc::on_liftoff("Database Config", |_rocket| {
            Box::pin(async move {
                println!("🚀 Blog API server starting up...");
                println!("📊 Database connection initialized");
            })
        }))
        .mount(
            "/api",
            routes![
                handlers::create_user,
                handlers::create_post,
                handlers::list_posts,
            ],
        )
}
