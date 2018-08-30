use hit_count::HitCount;
use rocket::State;
use rocket_contrib::Json;
use std::sync::atomic::Ordering;

#[get("/")]
fn route(hit_counter: State<HitCount>) -> Json {
    Json(json!({"favicon_hit_count": hit_counter.count.load(Ordering::Relaxed)}))
}
