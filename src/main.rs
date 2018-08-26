#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod gone;
mod greet;
mod hello;
mod hit_count;
mod index;
mod model;
mod stats;

use hit_count::HitCount;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::response::NamedFile;
use rocket::State;
use rocket::{Request, Response};
use rocket_contrib::Json;
use std::sync::atomic::{AtomicUsize, Ordering};

#[get("/")]
fn serve_favicon(hit_count: State<HitCount>) -> Option<NamedFile> {
    hit_count.count.fetch_add(1, Ordering::Acquire);
    NamedFile::open("favicon.ico").ok()
}

#[catch(404)]
fn not_found(request: &Request) -> Json {
    let message = match request.format() {
        Some(ref format) if !format.is_json() => format!(
            "Invalid format \"{}\" supplied, only application/json supported.",
            format
        ),
        _ => String::from("404 - Wrong route"),
    };
    Json(json!({"error": true, "message": message}))
}

struct ReplaceServerHeader;

impl Fairing for ReplaceServerHeader {
    fn info(&self) -> Info {
        Info {
            name: "Replace Server header",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        let mut server_name = "jens1o".to_owned();
        if cfg!(debug_assertions) {
            server_name.push_str(" [DEBUG]");
        }

        response.set_header(Header::new("Server", server_name));
    }
}

fn main() {
    rocket::ignite()
        .attach(ReplaceServerHeader {})
        .manage(HitCount {
            count: AtomicUsize::new(0),
        }).mount("/hello", routes![hello::route_json, hello::route_text])
        .mount("/greet", routes![greet::route])
        .mount("/", routes![index::route])
        .mount("/gone", routes![gone::route])
        .mount("/favicon.ico", routes![serve_favicon])
        .mount("/stats", routes![stats::route])
        .catch(catchers![not_found])
        .launch();
}
