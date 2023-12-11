// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Integer,
        task_name -> Text,
        task_start_time -> Text,
        task_end_time -> Text,
        finished -> Bool,
    }
}
