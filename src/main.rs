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
use rocket::response::NamedFile;
use rocket::State;
use std::sync::atomic::{AtomicUsize, Ordering};

#[get("/")]
fn serve_favicon(hit_count: State<HitCount>) -> Option<NamedFile> {
    hit_count.count.fetch_add(1, Ordering::Acquire);
    NamedFile::open("favicon.ico").ok()
}

fn main() {
    rocket::ignite()
        .manage(HitCount {
            count: AtomicUsize::new(0),
        }).mount("/hello", routes![hello::route_json, hello::route_text])
        .mount("/greet", routes![greet::route])
        .mount("/", routes![index::route])
        .mount("/gone", routes![gone::route])
        .mount("/favicon.ico", routes![serve_favicon])
        .mount("/stats", routes![stats::route])
        .launch();
}
