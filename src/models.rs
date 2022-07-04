use super::schema::tasks;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub done: bool,
}

#[derive(Insertable)]
#[table_name="tasks"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}