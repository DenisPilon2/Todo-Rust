extern crate diesel;

use actix_web::{web,  HttpResponse};
use crate::schema::tasks::dsl::*;
use self::diesel::prelude::*;
use crate::models::todo::Task;
use r2d2_diesel::ConnectionManager;
use diesel::pg::PgConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn done_task(path: web::Path<i32>,  pool: web::Data<DbPool>) -> HttpResponse {

    let conn = pool.get().expect("couldn't get db connection from pool");

    let post = diesel::update(tasks.find(path.into_inner()))
        .set(done.eq(true))
        .get_result::<Task>(&*conn)
        .expect(&format!("Unable to find task"));
    println!("Task done: {}", post.title);
    HttpResponse::Ok().json(post)
}
