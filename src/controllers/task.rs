use rocket::serde::{json::Json};
use std::collections::HashMap;
use uuid::Uuid;
use crate::dtos::{GetAllDto, Task};
use crate::databases::Tasks;
use rocket_db_pools::Connection;

#[get("/")]
pub async fn get_all(mut _db: Connection<Tasks>) -> Json<GetAllDto> {
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