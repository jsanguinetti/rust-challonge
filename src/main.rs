#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;

use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Tournament {
    id: i32,
}

#[get("/")]
fn index() -> Json<Tournament> {
    Json(Tournament { id: 32 })
}

#[post("/")]
fn create() -> JsonValue {
    json!({
        "message": "POST"
    })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/tournaments", routes![index, create])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
