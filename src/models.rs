use std::fmt;

use diesel::prelude::*;
use serde::{ Deserialize, Serialize };

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,

            "ID: {:?}, Title: {}, Description: {}, Completed: {}, Created At: {}",

            self.id.unwrap_or(0),

            self.title,

            self.description.as_deref().unwrap_or("None"),

            self.completed,

            self.created_at.map_or("None".to_string(), |dt| dt.to_string())
        )
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::todos)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub completed: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
}
