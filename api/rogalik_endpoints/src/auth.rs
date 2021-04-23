use rocket::Route;

#[post("/sign-up")]
fn sign_up() -> &'static str {
    "Hello there!"
}

pub fn get_routes() -> Vec<Route> {
    routes![sign_up]
}
