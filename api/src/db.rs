use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;

use super::diesel::prelude::*;
use super::models::User;
use super::schema::users;

#[database("diesel")]
struct DbConn(diesel::PgConnection);

#[get("/test")]
fn index(conn: DbConn) -> Json<Vec<User>> {
    let all_users = users::table.load(&*conn).expect("Could not load users");

    Json(all_users)
}

fn run_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    embed_migrations!("migrations");

    let conn = DbConn::get_one(&rocket).expect("database connection");

    match embedded_migrations::run(&*conn) {
        Ok(_) => Ok(rocket),
        Err(err) => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_attach("Diesel Postgres Stage", |rocket| {
        Ok(rocket
            .attach(DbConn::fairing())
            .attach(AdHoc::on_attach("Diesel Migrations", run_migrations))
            .mount("/", routes![index]))
    })
}
