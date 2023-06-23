use rocket::*;
use api::todo_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![
            todo_handler::list_todos_handler, 
            todo_handler::list_todo_handler,
        ])
}