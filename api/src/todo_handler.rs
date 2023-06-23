use shared::response_models::{Response, ResponseBody};
use application::todo::read;
use domain::models::Todo;
use rocket::get;
use rocket::response::status::NotFound;
// use rocket::serde::json::Josn;

#[get("/")]
pub fn list_todos_handler() -> String {
    let todos: Vec<Todo> = read::list_todos();
    let response = Response { body: ResponseBody::Todos(todos) };

    serde_json::to_string(&response).unwrap()
}

#[get("/todo/<todo_id>")]
pub fn list_todo_handler(todo_id: String) -> Result<String, NotFound<String>> {
    let todo = read::list_todo(todo_id)?;
    let response = Response { body: ResponseBody::Todo(todo) };

    Ok(serde_json::to_string(&response).unwrap())
}