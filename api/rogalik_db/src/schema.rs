table! {
    projects (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
    }
}

table! {
    tasks (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        project_id -> Uuid,
    }
}

table! {
    tasks_users (task_id, user_id) {
        task_id -> Uuid,
        user_id -> Uuid,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        password -> Varchar,
    }
}

joinable!(tasks -> projects (project_id));
joinable!(tasks_users -> tasks (task_id));
joinable!(tasks_users -> users (user_id));

allow_tables_to_appear_in_same_query!(
    projects,
    tasks,
    tasks_users,
    users,
);
