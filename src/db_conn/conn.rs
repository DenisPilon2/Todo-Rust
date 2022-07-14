use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::models::todo::{Post, NewPost};
use crate::schema;
use r2d2_diesel::ConnectionManager;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
  
    r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.")
  }

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    
    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(schema::tasks::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}