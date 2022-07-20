use crate::schema::tasks;

use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub done: bool,
}

#[derive(Insertable)]
#[table_name="tasks"]
pub struct NewTask<'a> {
    pub title: &'a String,
    pub body: &'a String,
}