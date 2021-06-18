use std::str::FromStr;

use rocket::{http::Status, response::status::Custom, Route};
use rocket_contrib::json::Json;

use rogalik_db::{diesel, diesel::prelude::*, models::TaskUser, schema::tasks_users, DbConn};
use uuid::Uuid;

#[get("/<task_id>")]
fn get_task_users(
    conn: DbConn,
    task_id: String,
) -> Custom<Result<Json<Vec<TaskUser>>, &'static str>> {
    let parsed_id = Uuid::from_str(task_id.as_str());

    if parsed_id.is_err() {
        return Custom(Status::UnprocessableEntity, Err("Invalid task ID provided"));
    }

    let task_users = tasks_users::table
        .filter(tasks_users::task_id.eq(parsed_id.unwrap()))
        .select((tasks_users::task_id, tasks_users::user_id))
        .load(&*conn);

    if task_users.is_err() {
        return Custom(
            Status::InternalServerError,
            Err("Could not fetch task's users, try again later"),
        );
    }

    Custom(Status::Ok, Ok(Json(task_users.unwrap())))
}

#[post(
    "/<task_id>",
    format = "application/json",
    data = "<assigned_users_ids>"
)]
fn assign_task_users(
    conn: DbConn,
    task_id: String,
    assigned_users_ids: Json<Vec<String>>,
) -> Custom<Result<(), &'static str>> {
    let parsed_id = Uuid::from_str(task_id.as_str());

    if parsed_id.is_err() {
        return Custom(Status::UnprocessableEntity, Err("Invalid task ID provided"));
    }

    let parsed_user_ids = assigned_users_ids
        .into_inner()
        .into_iter()
        .map(|string_id| Uuid::from_str(string_id.as_str()));

    if parsed_user_ids.clone().any(|id| id.is_err()) {
        return Custom(
            Status::UnprocessableEntity,
            Err("Invalid user IDs provided"),
        );
    }

    let parsed_user_ids: Vec<Uuid> = parsed_user_ids.map(|id| id.unwrap()).collect();

    let existing_task_users = tasks_users::table
        .filter(tasks_users::task_id.eq(parsed_id.unwrap()))
        .select((tasks_users::task_id, tasks_users::user_id))
        .load::<TaskUser>(&*conn);

    if existing_task_users.is_err() {
        return Custom(
            Status::InternalServerError,
            Err("Could not fetch task's users, try again later"),
        );
    }

    let existing_task_users = existing_task_users.unwrap();
    let existing_task_users_ids = existing_task_users
        .iter()
        .map(|task_user| task_user.user_id.clone())
        .collect::<Vec<Uuid>>();

    let mut task_users_to_remove: Vec<Uuid> = vec![];
    let mut task_users_to_add: Vec<Uuid> = vec![];

    for task_user in existing_task_users {
        if !parsed_user_ids.contains(&task_user.user_id) {
            task_users_to_remove.push(task_user.user_id);
        }
    }

    for new_user_id in parsed_user_ids {
        if !existing_task_users_ids.contains(&new_user_id) {
            task_users_to_add.push(new_user_id);
        }
    }

    let add_result = match task_users_to_add.clone().len() {
        0 => Ok(()),
        _ => diesel::insert_into(tasks_users::table)
            .values(
                &task_users_to_add
                    .iter()
                    .map(|&user_id| TaskUser {
                        task_id: parsed_id.unwrap(),
                        user_id,
                    })
                    .collect::<Vec<TaskUser>>(),
            )
            .get_result::<TaskUser>(&*conn)
            .map_or_else(|_| Err(()), |_| Ok(())),
    };

    let remove_result = match task_users_to_remove.clone().len() {
        0 => Ok(()),
        _ => diesel::delete(
            tasks_users::table.filter(tasks_users::user_id.eq_any(task_users_to_remove)),
        )
        .get_result::<TaskUser>(&*conn)
        .map_or_else(|_| Err(()), |_| Ok(())),
    };

    if add_result.is_err() || remove_result.is_err() {
        return Custom(
            Status::InternalServerError,
            Err("Error assigning the user, try again later"),
        );
    }

    Custom(Status::Ok, Ok(()))
}

pub fn get_routes() -> Vec<Route> {
    routes![get_task_users, assign_task_users]
}
