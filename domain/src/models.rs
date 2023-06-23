use crate::schema::todos;
use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub created_on: Option<String>,
    pub deadline: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = todos)]
pub struct NewTodo {
    pub title: String,
    pub description: Option<String>,
    pub created_on: Option<String>,
    pub deadline: Option<String>
}