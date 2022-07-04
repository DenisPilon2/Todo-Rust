use actix_web::{get, web, App, HttpResponse, HttpServer};
extern crate to_do;
extern crate diesel;

use self::to_do::*;
use self::models::*;
use self::diesel::prelude::*; 
use std::io::{stdin};


#[get("/tasks")]
async fn show_tasks() -> HttpResponse {
    use to_do::schema::tasks::dsl::*;

    let connection = establish_connection();
    let results = tasks
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");
    
    //HttpResponse::Ok().body("User detail:")
    //let json = results.parse::<jsondata::Json>().unwrap();
    //let data = serde_json::to_vec(&results);
    HttpResponse::Ok().json(results)
}
#[get("/add_task")]
async fn add_tasks() -> HttpResponse {
    let connection = establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
    let mut body = String::new();
    stdin().read_line(&mut body).unwrap();

    let post = create_post(&connection, title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);
    HttpResponse::Ok().body("Task added.")
}

#[get("/delete_task/{title}")]
async fn delete_tasks(path: web::Path<String>) -> HttpResponse {
    use to_do::schema::tasks::dsl::*;

    let connection = establish_connection();
    let pattern = format!("%{}%", path);
    let num_deleted = diesel::delete(tasks.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting task");

    HttpResponse::Ok().body(format!("Task deleted: {}", path))
}
#[get("/done/{id}")]
async fn done(path: web::Path<i32>) -> HttpResponse {
    use to_do::schema::tasks::dsl::*;

    let connection = establish_connection();

    let post = diesel::update(tasks.find(path.into_inner()))
        .set(done.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find task"));
    println!("Task done: {}", post.title);
    HttpResponse::Ok().body(format!("Task done"))
}

#[get("/undone/{id}")]
async fn undone(path: web::Path<i32>) -> HttpResponse {
    use to_do::schema::tasks::dsl::*;

    let connection = establish_connection();
 
    let post = diesel::update(tasks.find(path.into_inner()))
        .set(done.eq(false))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find task"));
    println!("Task undone: {}", post.title);
    HttpResponse::Ok().body(format!("Task undone"))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/todo")
                .service(show_tasks)
                .service(add_tasks)
                .service(delete_tasks)
                .service(done)
                .service(undone)
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";