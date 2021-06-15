#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod auth;
mod tasks;
mod users;

use rocket::fairing::AdHoc;

pub fn stage() -> AdHoc {
    AdHoc::on_attach("Diesel Postgres Stage", |rocket| {
        Ok(rocket
            .mount("/users", users::get_routes())
            .mount("/tasks", tasks::get_routes())
            .mount("/auth", auth::get_routes()))
    })
}
