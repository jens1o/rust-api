use rocket_contrib::json::JsonValue;

#[get("/<username>/<age>", format = "application/json", rank = 1)]
pub fn route_json(username: String, age: u8) -> JsonValue {
    let message = format!("Hello, {} year old named {}!", age, username);
    json!({ "message": message })
}

#[get("/<username>/<age>", format = "text/html", rank = 4)]
pub fn route_text(username: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, username)
}
