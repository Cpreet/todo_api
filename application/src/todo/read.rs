use domain::models::Todo;
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;

pub fn list_todo(todo_id: String) -> Result<Todo, NotFound<String>> {
    use domain::schema::todos;

    match todos::table.find(&todo_id).first::<Todo>(&mut establish_connection()){
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting todo with id {} - {}", todo_id, err))};
                Err(NotFound(serde_json::to_string(&response).unwrap()))
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}

pub fn list_todos() -> Vec<Todo> {
    use domain::schema::todos;

    match todos::table.select(todos::all_columns).load::<Todo>(&mut establish_connection()) {
        Ok(mut posts) => {
            posts.sort();
            posts
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}