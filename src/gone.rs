use rocket::http::Status;

#[get("/")]
pub fn route() -> Status {
    Status::raw(410)
}
