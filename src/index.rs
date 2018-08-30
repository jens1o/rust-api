use rocket_contrib::Json;

#[get("/")]
fn route() -> Json {
    Json(json!(
        {"message": "Hello World!"}
    ))
}
