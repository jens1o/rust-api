use rocket_contrib::json::{Json, JsonValue};

#[get("/")]
pub fn route() -> Json<JsonValue> {
    Json(json!(
        {"message": "Hello World!"}
    ))
}
