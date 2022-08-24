use rocket::serde::{json::Json};
use std::collections::HashMap;
use uuid::Uuid;
use crate::dtos::{GetAllDto, Task};

#[get("/")]
pub fn get_all() -> Json<GetAllDto> {
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