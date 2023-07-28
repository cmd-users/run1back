// src/models.rs
use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = example_table)]
pub struct Example {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

