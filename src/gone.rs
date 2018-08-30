use rocket::http::Status;
use rocket::response::Failure;

#[get("/")]
fn route() -> Failure {
    Failure(Status::raw(410))
}
