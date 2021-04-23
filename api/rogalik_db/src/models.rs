extern crate uuid;

use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct UserWithoutId {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}
