use hit_count::HitCount;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use std::sync::atomic::Ordering;

#[get("/")]
pub fn route(hit_counter: State<HitCount>) -> Json<JsonValue> {
    Json(json!({"favicon_hit_count": hit_counter.count.load(Ordering::Relaxed)}))
}
