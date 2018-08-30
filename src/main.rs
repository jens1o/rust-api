#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate ctrlc;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod fairings;
mod gone;
mod greet;
mod hello;
mod hit_count;
mod index;
mod model;
mod stats;

use hit_count::HitCount;
use rocket::response::NamedFile;
use rocket::Request;
use rocket::Rocket;
use rocket::State;
use rocket_contrib::Json;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

const HIT_COUNT_FILE: &'static str = "hit_count.txt";

#[get("/")]
fn serve_favicon(hit_count: State<HitCount>) -> Option<NamedFile> {
    println!(
        "Favicon hits: {}",
        hit_count.count.fetch_add(1, Ordering::Acquire) + 1
    );
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

fn rocket(hit_count: HitCount) -> Rocket {
    rocket::ignite()
        .attach(fairings::ReplaceServerHeader)
        .manage(hit_count)
        .mount("/hello", routes![hello::route_json, hello::route_text])
        .mount("/greet", routes![greet::route])
        .mount("/", routes![index::route])
        .mount("/gone", routes![gone::route])
        .mount("/favicon.ico", routes![serve_favicon])
        .mount("/stats", routes![stats::route])
        .catch(catchers![not_found])
}

fn main() {
    let hit_count_path = Path::new(HIT_COUNT_FILE);
    let mut hit_count: usize = 0;

    if hit_count_path.exists() {
        // read hit count from file
        let file = File::open(HIT_COUNT_FILE);
        let mut file_content = String::new();

        file.expect("file exists")
            .read_to_string(&mut file_content)
            .unwrap();

        hit_count = file_content.parse().unwrap();
    }

    let hit_count = HitCount {
        count: Arc::new(AtomicUsize::new(hit_count)),
    };

    let count = Arc::clone(&hit_count.count);

    ctrlc::set_handler(move || {
        println!("Exiting!");
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(HIT_COUNT_FILE)
            .unwrap();

        write!(file, "{}", count.load(Ordering::SeqCst).to_string());
    }).unwrap();

    rocket(hit_count).launch();
}
