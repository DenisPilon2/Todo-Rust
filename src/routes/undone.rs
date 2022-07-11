use actix_web::{web,  HttpResponse};
extern crate diesel;
use crate::db_conn::conn;
use crate::schema::tasks::dsl::*;
use self::diesel::prelude::*;
use crate::models::todo::Post;

pub async fn undone_task(path: web::Path<i32>) -> HttpResponse {

    let connection = conn::establish_connection();
 
    let post = diesel::update(tasks.find(path.into_inner()))
        .set(done.eq(false))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find task"));
    println!("Task undone: {}", post.title);
    HttpResponse::Ok().body(format!("Task undone"))
}