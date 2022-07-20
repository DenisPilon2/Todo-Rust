use crate::models::todo::{NewTask, Task};
use crate::schema;

use actix_web::{HttpResponse};
use actix_web::{web};
use r2d2_diesel::ConnectionManager;
use diesel::pg::PgConnection;
use serde::Deserialize;
use diesel::prelude::*;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
pub struct FormData {
    title: String,
    body: String,
}

pub async fn add_task(pool: web::Data<DbPool>, form: web::Form<FormData>) -> HttpResponse {

    let conn = pool.get().expect("couldn't get db connection from pool");
    
    let new_post = NewTask {
        title: &form.title,
        body: &form.body,
    };

    let new_task:Task = diesel::insert_into(schema::tasks::table)
        .values(&new_post)
        .get_result(&*conn)
        .expect("Error saving new task");
    
    println!("New task added: {}", new_task.title);
    HttpResponse::Ok().json(new_task)
}

