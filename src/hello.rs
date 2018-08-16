extern crate rocket;

use rocket_contrib::Json;

#[get("/<username>/<age>", format = "application/json")]
fn route_json(username: String, age: u8) -> Json {
    let message = format!("Hello, {} year old named {}!", age, username);
    Json(json!({ "message": message }))
}

#[get("/<username>/<age>", format = "text/html")]
fn route_text(username: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, username)
}
