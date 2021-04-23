#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rogalik_db;
use rogalik_endpoints;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

fn main() {
    rocket::ignite()
        .attach(rogalik_db::stage())
        .attach(rogalik_endpoints::stage())
        .mount("/", routes![index])
        .launch();
}
