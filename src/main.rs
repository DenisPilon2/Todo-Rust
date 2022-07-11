pub mod schema;
pub mod models;
mod db_conn;

use actix_web::{web, App, HttpServer};

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/show_tasks", web::get().to(routes::show_tasks::show_tasks))
            .route("/add_task", web::get().to(routes::add_task::add_task))
            .route("/delete_task/{task_title}", web::get().to(routes::delete_task::delete_task))
            .route("/done/{task_id}", web::get().to(routes::done::done_task))
            .route("/undone/{task_id}", web::get().to(routes::undone::undone_task))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

