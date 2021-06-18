use std::str::FromStr;

use rocket::{http::Status, response::status::Custom, Route};
use rocket_contrib::json::Json;

use rogalik_db::{
    diesel,
    diesel::prelude::*,
    models::{EditedProject, NewProject, Project, Task},
    schema::{projects, tasks},
    DbConn,
};
use uuid::Uuid;

#[get("/")]
fn index(conn: DbConn) -> Json<Vec<Project>> {
    let all_projects = projects::table
        .select((projects::id, projects::name, projects::description))
        .load(&*conn)
        .expect("Could not load projects");

    Json(all_projects)
}

#[post("/", format = "application/json", data = "<new_project>")]
fn add(
    conn: DbConn,
    new_project: Option<Json<NewProject>>,
) -> Custom<Result<Json<Project>, &'static str>> {
    if new_project.is_none() {
        return Custom(Status::BadRequest, Err("Invalid data provided"));
    }

    let parsed_project = new_project.unwrap().into_inner();

    diesel::insert_into(projects::table)
        .values(&parsed_project)
        .returning((projects::id, projects::name, projects::description))
        .get_result(&*conn)
        .map_or_else(
            |_| {
                Custom(
                    Status::InternalServerError,
                    Err("Error adding the user, try again later"),
                )
            },
            |created_project: Project| Custom(Status::Created, Ok(Json(created_project))),
        )
}

#[patch("/<id>", format = "application/json", data = "<edited_project>")]
fn edit(
    conn: DbConn,
    edited_project: Option<Json<EditedProject>>,
    id: String,
) -> Custom<Result<Json<Project>, &'static str>> {
    if edited_project.is_none() {
        return Custom(Status::BadRequest, Err("Invalid data provided"));
    }

    let parsed_id = Uuid::from_str(id.as_str());

    if parsed_id.is_err() {
        return Custom(
            Status::UnprocessableEntity,
            Err("Invalid project ID provided"),
        );
    }

    let parsed_project = edited_project.unwrap().into_inner();

    diesel::update(projects::table.find(parsed_id.unwrap()))
        .set(&parsed_project)
        .returning((projects::id, projects::name, projects::description))
        .get_result(&*conn)
        .map_or_else(
            |_| {
                Custom(
                    Status::InternalServerError,
                    Err("Error adding the user, try again later"),
                )
            },
            |created_project: Project| Custom(Status::Created, Ok(Json(created_project))),
        )
}

#[delete("/<id>")]
fn delete(conn: DbConn, id: String) -> Custom<&'static str> {
    let parsed_id = Uuid::from_str(id.as_str());

    if parsed_id.is_err() {
        return Custom(Status::UnprocessableEntity, "Invalid project ID provided");
    }

    let removal_result =
        diesel::delete(projects::table.filter(projects::id.eq(parsed_id.unwrap()))).execute(&*conn);

    if removal_result.is_err() {
        return Custom(
            Status::InternalServerError,
            "Error removing project, try again later",
        );
    }

    if removal_result.unwrap().eq(&0) {
        return Custom(Status::NotFound, "Project not found");
    }

    Custom(Status::NoContent, "")
}

#[get("/<id>")]
fn get_project(conn: DbConn, id: String) -> Custom<Result<Json<Project>, &'static str>> {
    let parsed_id = Uuid::from_str(id.as_str());

    if parsed_id.is_err() {
        return Custom(
            Status::UnprocessableEntity,
            Err("Invalid project ID provided"),
        );
    }

    let project = projects::table
        .find(parsed_id.unwrap())
        .select((projects::id, projects::name, projects::description))
        .first(&*conn);

    if project.is_err() {
        return Custom(Status::NotFound, Err("Project not found"));
    }

    Custom(Status::Ok, Ok(Json(project.unwrap())))
}

#[get("/<id>/tasks")]
fn get_project_tasks(conn: DbConn, id: String) -> Custom<Result<Json<Vec<Task>>, &'static str>> {
    let parsed_id = Uuid::from_str(id.as_str());

    if parsed_id.is_err() {
        return Custom(
            Status::UnprocessableEntity,
            Err("Invalid project ID provided"),
        );
    }

    if projects::table
        .find(parsed_id.unwrap())
        .first::<Project>(&*conn)
        .is_err()
    {
        return Custom(Status::NotFound, Err("Project not found"));
    }

    let project_tasks = tasks::table
        .filter(tasks::project_id.eq(parsed_id.unwrap()))
        .select((
            tasks::id,
            tasks::name,
            tasks::description,
            tasks::project_id,
        ))
        .load(&*conn);

    if project_tasks.is_err() {
        return Custom(
            Status::InternalServerError,
            Err("Could not fetch project tasks, try again later"),
        );
    }

    Custom(Status::Ok, Ok(Json(project_tasks.unwrap())))
}

pub fn get_routes() -> Vec<Route> {
    routes![index, add, edit, delete, get_project, get_project_tasks]
}
