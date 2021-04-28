use blake3;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::Json;

use rogalik_db::{
    diesel, diesel::prelude::*, models::NewUser, models::User, schema::users, DbConn,
};

#[post("/sign-up", format = "application/json", data = "<new_user>")]
fn sign_up(conn: DbConn, new_user: Json<NewUser>) -> Result<Json<User>, Status> {
    let mut parsed_user = new_user.into_inner();

    let hashed_password = blake3::hash(parsed_user.password.as_bytes());

    parsed_user.password = hashed_password.to_hex().as_str().to_owned();

    diesel::insert_into(users::table)
        .values(&parsed_user)
        .returning((users::id, users::email, users::first_name, users::last_name))
        .get_result(&*conn)
        .map(|created_user| Json(created_user))
        .map_err(|_| Status::BadRequest)
}

pub fn get_routes() -> Vec<Route> {
    routes![sign_up]
}
