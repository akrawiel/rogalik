extern crate uuid;

use crate::schema::{projects, tasks, tasks_users, users};
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

#[derive(Identifiable, Queryable, Deserialize, Serialize)]
#[table_name = "tasks"]
pub struct Task {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub project_id: Option<uuid::Uuid>,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "tasks"]
pub struct NewTask {
    pub name: String,
    pub description: Option<String>,
    pub project_id: Option<uuid::Uuid>,
}

#[derive(Insertable, Queryable, Deserialize, Serialize)]
#[table_name = "tasks_users"]
pub struct TaskUser {
    pub task_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
}

#[derive(Identifiable, Queryable, Deserialize, Serialize)]
#[table_name = "projects"]
pub struct Project {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
}
