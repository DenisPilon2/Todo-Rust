extern crate to_do;
extern crate diesel;

use self::to_do::*;
use self::models::*;
use self::diesel::prelude::*; 

pub fn main() {
    use to_do::schema::tasks::dsl::*;

    let connection = establish_connection();
    let results = tasks
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}