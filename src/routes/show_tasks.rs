extern crate diesel;

use crate::{models::todo::Task};
use crate::schema::tasks::{dsl::*, self};
use actix_web::{HttpResponse, Responder};
use actix_web::{web};
use self::diesel::prelude::*;
use r2d2_diesel::ConnectionManager;
use diesel::pg::PgConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn show_tasks(pool: web::Data<DbPool>) -> impl Responder{
    
    let conn = pool.get().expect("couldn't get db connection from pool");

    match tasks
        .order(tasks::id.desc())
        .load::<Task>(&*conn)
        {
            Ok(results) => return HttpResponse::Ok().json(results),
            Err(_) => return HttpResponse::NotFound().finish(),
        } 
}