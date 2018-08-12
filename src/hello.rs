extern crate rocket;

use rocket_contrib::Json;

#[get("/<username>/<age>")]
fn route(username: String, age: u8) -> Json {
    let message = format!("Hello, {} year old named {}!", age, username);
    Json(json!({ "message": message }))
}
