extern crate diesel;

use crate::schema::tasks::dsl::*;
use crate::models::todo::Task;
use actix_web::{web,  HttpResponse};
use self::diesel::prelude::*;
use r2d2_diesel::ConnectionManager;
use diesel::pg::PgConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn undone_task(path: web::Path<i32>, pool: web::Data<DbPool>) -> HttpResponse {

    let conn = pool.get().expect("couldn't get db connection from pool");

    let post = diesel::update(tasks.find(path.into_inner()))
        .set(done.eq(false))
        .get_result::<Task>(&*conn)
        .expect(&format!("Unable to find task"));
    println!("Task undone: {}", post.title);
    HttpResponse::Ok().json(post)
}