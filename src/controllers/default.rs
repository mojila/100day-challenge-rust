use rocket::serde::{json::Json};
use crate::dtos::Response;

#[get("/")]
pub fn index() -> Json<Response> {
    Json(Response {
        message: String::from("Welcome to Todo API v1")
    })
}