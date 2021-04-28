#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
pub extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

pub mod models;
pub mod schema;

use rocket::fairing::AdHoc;
use rocket::Rocket;

#[database("diesel")]
pub struct DbConn(diesel::PgConnection);

fn run_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    embed_migrations!("../migrations");

    let conn = DbConn::get_one(&rocket).expect("database connection");

    match embedded_migrations::run(&*conn) {
        Ok(_) => Ok(rocket),
        Err(_) => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_attach("Diesel Postgres Stage", |rocket| {
        Ok(rocket
            .attach(DbConn::fairing())
            .attach(AdHoc::on_attach("Diesel Migrations", run_migrations)))
    })
}
