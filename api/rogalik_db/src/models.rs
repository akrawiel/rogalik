extern crate uuid;

use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Queryable, Identifiable, QueryableByName, Deserialize, Serialize)]
#[table_name = "users"]
pub struct FullUser {
    pub id: uuid::Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

#[derive(QueryableByName, Deserialize, Serialize)]
#[table_name = "users"]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}
