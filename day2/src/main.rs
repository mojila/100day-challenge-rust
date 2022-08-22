use rocket::serde::{Serialize, json::Json};
use sysinfo::{get_current_pid, PidExt};
use uptime_lib;
use chrono;
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Heloo"
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct HealtCheck {
    pid: u32,
    uptime: u128,
    message: String,
    timestamp: i64
}

#[get("/")]
fn health_check() -> Json<HealtCheck> {
    let pid = get_current_pid().unwrap().as_u32();
    let uptime = uptime_lib::get().unwrap().as_millis();
    let timestamp = chrono::offset::Local::now().timestamp();
    
    return Json(HealtCheck {
        pid,
        uptime,
        message: String::from("OK"),
        timestamp
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/healthcheck", routes![health_check])
}