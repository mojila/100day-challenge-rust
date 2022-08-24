use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub message: String
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub is_done: bool
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetAllDto {
    pub data: Vec<Task>,
    pub message: String
}