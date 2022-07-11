use actix_web::{HttpResponse};
use crate::db_conn::conn;


use std::io::stdin;

pub async fn add_task() -> HttpResponse {
    let connection = conn::establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
    let mut body = String::new();
    stdin().read_line(&mut body).unwrap();

    let post = conn::create_post(&connection, title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);
    HttpResponse::Ok().body("Task added.")
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
