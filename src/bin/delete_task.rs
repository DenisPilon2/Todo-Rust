extern crate to_do;
extern crate diesel;

use self::diesel::prelude::*;
use self::to_do::*;
use std::env::args;

fn main() {
    use to_do::schema::tasks::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(tasks.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}