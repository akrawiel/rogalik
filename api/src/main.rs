#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod db;
mod models;
mod schema;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

fn main() {
    rocket::ignite()
        .attach(db::stage())
        .mount("/", routes![index])
        .launch();
}
