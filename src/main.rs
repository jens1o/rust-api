#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod gone;
mod greet;
mod hello;
mod index;
mod model;

use rocket::response::NamedFile;

#[get("/")]
fn serve_favicon() -> Option<NamedFile> {
    NamedFile::open("favicon.ico").ok()
}

fn main() {
    rocket::ignite()
        .mount("/hello", routes![hello::route])
        .mount("/greet", routes![greet::route])
        .mount("/", routes![index::route])
        .mount("/gone", routes![gone::route])
        .mount("/favicon.ico", routes![serve_favicon])
        .launch();
}
