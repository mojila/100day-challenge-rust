#[macro_use] extern crate rocket;

mod controllers;
mod dtos;

use controllers::default as default;
use controllers::task as task;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![default::index])
        .mount("/tasks", routes![task::get_all])
}
