use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::models::todo::{Post, NewPost};
use crate::schema;

pub fn establish_connection() -> PgConnection{
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    //use schema::tasks;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(schema::tasks::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}