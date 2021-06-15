use std::str::FromStr;

use rocket::{http::Status, response::status::Custom, Route};
use rocket_contrib::json::Json;

use rogalik_db::{
    diesel,
    diesel::prelude::*,
    models::{NewTask, Task},
    schema::tasks,
    DbConn,
};
use uuid::Uuid;

#[get("/")]
fn index(conn: DbConn) -> Json<Vec<Task>> {
    let all_tasks = tasks::table
        .select((
            tasks::id,
            tasks::name,
            tasks::description,
            tasks::project_id,
        ))
        .load(&*conn)
        .expect("Could not load tasks");

    Json(all_tasks)
}

#[post("/", format = "application/json", data = "<new_task>")]
fn add(conn: DbConn, new_task: Option<Json<NewTask>>) -> Custom<Result<Json<Task>, &'static str>> {
    if new_task.is_none() {
        return Custom(Status::BadRequest, Err("Invalid data provided"));
    }

    let parsed_task = new_task.unwrap().into_inner();

    diesel::insert_into(tasks::table)
        .values(&parsed_task)
        .returning((
            tasks::id,
            tasks::name,
            tasks::description,
            tasks::project_id,
        ))
        .get_result(&*conn)
        .map_or_else(
            |_| {
                Custom(
                    Status::InternalServerError,
                    Err("Error adding the user, try again later"),
                )
            },
            |created_task: Task| Custom(Status::Created, Ok(Json(created_task))),
        )
}

#[delete("/<id>")]
fn delete(conn: DbConn, id: Option<String>) -> Custom<&'static str> {
    if id.is_none() {
        return Custom(Status::BadRequest, "Task ID not provided");
    }

    let parsed_id = Uuid::from_str(id.unwrap().as_str());

    if parsed_id.is_err() {
        return Custom(Status::UnprocessableEntity, "Invalid task ID provided");
    }

    let removal_result =
        diesel::delete(tasks::table.filter(tasks::id.eq(parsed_id.unwrap()))).execute(&*conn);

    if removal_result.is_err() {
        return Custom(
            Status::InternalServerError,
            "Error removing task, try again later",
        );
    }

    if removal_result.unwrap().eq(&0) {
        return Custom(Status::NotFound, "Task not found");
    }

    Custom(Status::NoContent, "")
}

pub fn get_routes() -> Vec<Route> {
    routes![index, add, delete]
}
