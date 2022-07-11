use actix_web::{web,  HttpResponse};
extern crate diesel;
use crate::db_conn::conn;
use crate::schema::tasks::dsl::*;
use self::diesel::prelude::*;

pub async fn delete_task(path: web::Path<String>) -> HttpResponse {

    let connection = conn::establish_connection();
    let pattern = format!("%{}%", path);
    /*let num_deleted = */diesel::delete(tasks.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting task");

    HttpResponse::Ok().body(format!("Task deleted: {}", path))
}