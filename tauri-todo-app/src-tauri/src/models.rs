/*
 * Description:
 * Created: 2023-12-11 15:31:19
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */
use super::schema::tasks;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize)]
#[serde(rename_all = "camelCase")] // 序列化为驼峰
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub task_name: String,
    pub task_start_time: String,
    pub task_end_time: String,
    pub finished: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = tasks)]

pub struct NewTask<'a> {
    pub task_name: &'a str,
    pub task_start_time: &'a str,
    pub task_end_time: &'a str,
}
