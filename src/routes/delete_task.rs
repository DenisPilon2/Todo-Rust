extern crate diesel;

use actix_web::{web,  HttpResponse};
use crate::schema::tasks::dsl::*;
use self::diesel::prelude::*;
use r2d2_diesel::ConnectionManager;
use diesel::pg::PgConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn delete_task(path: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {

    let conn = pool.get().expect("couldn't get db connection from pool");

    let pattern = format!("%{}%", path);
    diesel::delete(tasks.filter(title.like(pattern)))
        .execute(&*conn)
        .expect("Error deleting task");

    println!("Task deleted: {}", path);
    HttpResponse::Ok().body(format!("Task deleted: {}", path))
}