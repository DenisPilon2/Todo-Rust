//mod db_conn;

use actix_web::{HttpResponse, Responder};
extern crate diesel;

use crate::{models::todo::Post, db_conn::conn};


use crate::schema::tasks::{dsl::*, self};
use self::diesel::prelude::*;


pub async fn show_tasks() -> impl Responder{
    
    let connection = conn::establish_connection();
    match tasks
        .order(tasks::id.desc())
        .load::<Post>(&connection)
        {
            Ok(results) => return HttpResponse::Ok().json(results),
            Err(_) => return HttpResponse::NotFound().finish(),
        } 
}