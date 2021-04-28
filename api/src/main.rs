#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;

use rogalik_db;
use rogalik_endpoints;

fn main() {
    rocket::ignite()
        .attach(rogalik_db::stage())
        .attach(rogalik_endpoints::stage())
        .launch();
}
