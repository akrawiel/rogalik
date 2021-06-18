#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod auth;
mod projects;
mod tasks;
mod tasks_users;
mod users;

use rocket::fairing::AdHoc;

pub fn stage() -> AdHoc {
    AdHoc::on_attach("Diesel Postgres Stage", |rocket| {
        Ok(rocket
            .mount("/projects", projects::get_routes())
            .mount("/users", users::get_routes())
            .mount("/tasks", tasks::get_routes())
            .mount("/tasks_users", tasks_users::get_routes())
            .mount("/auth", auth::get_routes()))
    })
}
