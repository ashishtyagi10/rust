#![feature(plugin)]
#[macro_use] extern crate rocket;

use rocket::{Rocket, Build};

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "hello, world!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![hello])
        .mount("/hi", routes![world])
        .mount("/hello", routes![world])
}
