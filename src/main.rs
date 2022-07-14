#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

mod db_conn;
mod routes;

use actix_web::{web, App, HttpServer};
use crate::db_conn::conn;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().app_data(web::Data::new(conn::get_connection_pool().clone()))
            .route("/show_tasks", web::get().to(routes::show_tasks::show_tasks))
            .route("/add_task", web::get().to(routes::add_task::add_task))
            .route("/delete_task/{task_title}", web::get().to(routes::delete_task::delete_task))
            .route("/done/{task_id}", web::get().to(routes::done::done_task))
            .route("/undone/{task_id}", web::get().to(routes::undone::undone_task))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

