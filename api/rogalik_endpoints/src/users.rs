use rocket::Route;
use rocket_contrib::json::Json;

use rogalik_db::{diesel::prelude::*, models::User, schema::users, DbConn};

#[get("/")]
fn index(conn: DbConn) -> Json<Vec<User>> {
    let all_users = users::table.load(&*conn).expect("Could not load users");

    Json(all_users)
}

pub fn get_routes() -> Vec<Route> {
    routes![index]
}
