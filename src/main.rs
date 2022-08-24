#[macro_use] extern crate rocket;

mod controllers;
mod dtos;
mod databases;

use controllers::default as default;
use controllers::task as task;
use databases::Tasks;
use rocket_db_pools::Database;

#[launch]
fn rocket() -> _ {
    let figments  = rocket::Config::figment()
        .merge(("databases.sqlite_tasks", rocket_db_pools::Config {
            url: "db.sqlite".into(),
            min_connections: None,
            max_connections: 1024,
            connect_timeout: 3,
            idle_timeout: None
        }));

    rocket::custom(figments)
        .attach(Tasks::init())
        .mount("/", routes![default::index])
        .mount("/tasks", routes![task::get_all])
}
