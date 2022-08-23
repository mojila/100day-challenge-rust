use std::collections::HashMap;

use rocket::serde::{Serialize, Deserialize, json::Json};
use uuid::Uuid;
#[macro_use] extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task {
    id: String,
    title: String,
    description: String,
    is_done: bool
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Response {
    message: String
}

#[get("/")]
fn index() -> Json<Response> {
    Json(Response {
        message: String::from("Welcome to Todo API v1")
    })
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct GetAllDto {
    data: Vec<Task>,
    message: String
}

#[get("/")]
fn get_all() -> Json<GetAllDto> {
    let mut tasks = HashMap::new();
    let id = Uuid::new_v4().to_string();
    
    tasks.insert(id.clone(), Task {
        id: id.clone(),
        title: String::from("Task A"),
        description: String::from("desc"),
        is_done: false
    });

    Json(GetAllDto {
        data: tasks.into_values().collect(),
        message: String::from("Successfully get tasks")
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/tasks", routes![get_all])
}
