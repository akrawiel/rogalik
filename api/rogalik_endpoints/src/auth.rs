use std::str::FromStr;

use blake3;
use diesel::result::{DatabaseErrorKind, Error, Error::DatabaseError};
use rocket::{
    http::{Cookie, Cookies, Status},
    response::status::Custom,
    Route,
};
use rocket_contrib::json::Json;
use uuid::Uuid;

use rogalik_db::{
    diesel,
    diesel::prelude::*,
    models::{FullUser, LoginUser, NewUser, User},
    schema::users,
    DbConn,
};

#[post("/sign-up", format = "application/json", data = "<new_user>")]
fn sign_up(
    conn: DbConn,
    new_user: Option<Json<NewUser>>,
    mut cookies: Cookies,
) -> Custom<Result<Json<User>, &'static str>> {
    if new_user.is_none() {
        return Custom(Status::BadRequest, Err("Invalid data provided"));
    }

    let parsed_user = new_user.unwrap().into_inner();

    let user_with_hashed_password = NewUser {
        password: blake3::hash(parsed_user.password.as_bytes())
            .to_hex()
            .as_str()
            .to_owned(),
        ..parsed_user
    };

    diesel::insert_into(users::table)
        .values(&user_with_hashed_password)
        .returning((users::id, users::email, users::first_name, users::last_name))
        .get_result(&*conn)
        .map_or_else(
            |error| match error {
                DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                    Custom(Status::Forbidden, Err("User already exists"))
                }
                _ => Custom(
                    Status::InternalServerError,
                    Err("Error adding the user, try again later"),
                ),
            },
            |created_user: User| {
                cookies.add_private(Cookie::new("user_id", created_user.id.urn().to_string()));
                Custom(Status::Created, Ok(Json(created_user)))
            },
        )
}

#[post("/sign-in", format = "application/json", data = "<login_user>")]
fn sign_in(
    conn: DbConn,
    login_user: Option<Json<LoginUser>>,
    mut cookies: Cookies,
) -> Custom<&'static str> {
    let invalid_login_response = Custom(Status::Unauthorized, "Invalid login data provided");

    if login_user.is_none() {
        return invalid_login_response;
    }

    let login_user_unwrapped = login_user.unwrap();

    let found_user: Result<FullUser, Error> = users::table
        .filter(users::email.eq(&login_user_unwrapped.email))
        .select((
            users::id,
            users::email,
            users::first_name,
            users::last_name,
            users::password,
        ))
        .first(&*conn);

    if found_user.is_err() {
        return invalid_login_response;
    }

    let found_user_unwrapped: FullUser = found_user.unwrap();
    let login_user_password = blake3::hash(&login_user_unwrapped.password.as_bytes())
        .to_hex()
        .as_str()
        .to_owned();

    if login_user_password != found_user_unwrapped.password {
        return invalid_login_response;
    }

    cookies.add_private(Cookie::new(
        "user_id",
        found_user_unwrapped.id.urn().to_string(),
    ));

    Custom(Status::Ok, "Logged in")
}

#[get("/sign-out", format = "application/json")]
fn sign_out(mut cookies: Cookies) -> Custom<&'static str> {
    if cookies.get_private("user_id").is_some() {
        cookies.remove_private(Cookie::named("user_id"));
        return Custom(Status::Ok, "Logged out");
    }

    Custom(Status::Unauthorized, "Not logged in")
}

#[get("/me", format = "application/json")]
fn me(conn: DbConn, mut cookies: Cookies) -> Custom<Result<Json<User>, &'static str>> {
    let user_id_cookie = cookies.get_private("user_id");

    if user_id_cookie.is_none() {
        return Custom(Status::Unauthorized, Err("Not logged in"));
    }

    let parsed_user_id = Uuid::from_str(user_id_cookie.unwrap().value());

    if parsed_user_id.is_err() {
        return Custom(Status::Unauthorized, Err("Not logged in"));
    }

    users::table
        .filter(users::id.eq(parsed_user_id.unwrap()))
        .select((users::id, users::email, users::first_name, users::last_name))
        .first(&*conn)
        .map_or_else(
            |_| Custom(Status::Unauthorized, Err("Not logged in")),
            |logged_in_user| Custom(Status::Ok, Ok(Json(logged_in_user))),
        )
}

pub fn get_routes() -> Vec<Route> {
    routes![sign_up, sign_in, sign_out, me]
}
