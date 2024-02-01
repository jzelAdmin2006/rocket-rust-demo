#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use rocket::serde::Serialize;

#[derive(Serialize)]
struct SumResult {
    result: i32,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/ping")]
fn ping() -> Value {
    json!({
        "message": "pong".to_string(),
    })
}

#[get("/plus?<a>&<b>")]
fn plus(a: Option<i32>, b: Option<i32>) -> Result<Json<SumResult>, status::Custom<String>> {
    match (a, b) {
        (Some(a_val), Some(b_val)) => {
            Ok(Json(SumResult { result: a_val + b_val }))
        },
        _ => Err(status::Custom(Status::BadRequest, "invalid parameter".to_string())),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
        .mount("/api", routes![ping, plus])
}
