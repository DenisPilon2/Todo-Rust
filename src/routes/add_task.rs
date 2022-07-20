use crate::db_conn::conn;

use actix_web::{HttpResponse};
use actix_web::{web};
use r2d2_diesel::ConnectionManager;
use diesel::pg::PgConnection;
use std::io::stdin;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn add_task(pool: web::Data<DbPool>) -> HttpResponse {

    let conn = pool.get().expect("couldn't get db connection from pool");
    
    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
    let mut body = String::new();
    stdin().read_line(&mut body).unwrap();

    let post = conn::create_post(&conn, title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);
    HttpResponse::Ok().json(post)
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
