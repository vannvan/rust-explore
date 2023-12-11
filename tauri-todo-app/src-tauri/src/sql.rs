/*
 * Description:
 * Created: 2023-12-11 16:32:52
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use crate::Task;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::NewTask;

pub struct DbTools {}

impl DbTools {
    /**
     * 连接数据库
     */
    pub fn establish_connection() -> SqliteConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }

    /**
     * 新建任务
     */
    pub fn create_task(conn: &mut SqliteConnection, new_task: &NewTask) -> Task {
        use crate::schema::tasks;

        diesel::insert_into(tasks::table)
            .values(&*new_task)
            .returning(Task::as_returning())
            .get_result(conn)
            .expect("数据插入失败")
    }
    /**
     * 查询所有数据
     */
    pub fn get_all_tasks() -> Vec<Task> {
        let connection = &mut Self::establish_connection();
        use crate::schema::tasks::dsl::*;

        let results = tasks
            // .limit(5)
            .select(Task::as_select())
            .load(connection)
            .expect("获取列表失败");
        results
    }

    /**
     * 记录是否存在
     */
    fn is_exit_record(target_id: &i32) -> bool {
        use crate::schema::tasks::dsl::*;
        let connection = &mut Self::establish_connection();

        let task = tasks
            .find(target_id)
            .select(Task::as_select())
            .first(connection)
            .optional(); // This allows for returning an Option<Post>, otherwise it will throw an error

        match task {
            Ok(Some(task)) => {
                println!("记录存在 {}", task.id);
                return true;
            }
            Ok(None) => {
                println!("记录不存在 {}", target_id);
                return false;
            }
            Err(_) => {
                println!("记录查询出错 {}", target_id);
                return false;
            }
        }
    }

    /**
     * 更新完成状态
     */
    pub fn update_finished_by_id(target_id: &i32, target: &bool) -> i32 {
        use crate::schema::tasks::dsl::*;
        let connection = &mut Self::establish_connection();

        let is_exit = Self::is_exit_record(target_id);

        if !is_exit {
            return -1;
        }

        let res = diesel::update(tasks.find(target_id))
            .set(finished.eq(target))
            .returning(Task::as_returning())
            .get_result(connection)
            .unwrap();

        if res.task_name.is_empty() {
            0
        } else {
            1
        }
    }

    /**
     * 更新数据
     */
    pub fn update_task_by_id(target_id: &i32, task_info: &NewTask) -> i32 {
        use crate::schema::tasks::dsl::*;
        let connection = &mut Self::establish_connection();

        let is_exit = Self::is_exit_record(target_id);

        if !is_exit {
            return -1;
        }
        let res = diesel::update(tasks.find(target_id))
            .set((
                task_name.eq(task_info.task_name),
                task_start_time.eq(task_info.task_start_time),
                task_end_time.eq(task_info.task_end_time),
            ))
            .returning(Task::as_returning())
            .get_result(connection)
            .unwrap();
        println!("更新记录结果,{:?}", res);
        if res.task_name.is_empty() {
            0
        } else {
            1
        }
    }

    /**
     * 删除一条记录
     */
    pub fn delete_one_record_by_id(target_id: &i32) -> i32 {
        use crate::schema::tasks::dsl::*;

        let is_exit = Self::is_exit_record(target_id);

        if !is_exit {
            return -1;
        }

        let connection = &mut Self::establish_connection();
        let num_deleted = diesel::delete(tasks.filter(id.eq(target_id)))
            .execute(connection)
            .expect("Error deleting posts");
        num_deleted.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    #[test]
    fn test_connect_db() {
        DbTools::establish_connection();
    }

    #[test]
    fn test_insert_data() {
        let s: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>();

        let connection = &mut DbTools::establish_connection();

        let new_data = NewTask {
            task_name: &String::from(format!("代办任务-{}", s)),
            task_start_time: &String::from("2023.12.12"),
            task_end_time: &String::from("2023.12.16"),
        };

        let post = DbTools::create_task(connection, &new_data);
        println!("\nSaved draft {} with id {}", post.task_name, post.id);
    }

    #[test]
    fn test_get_all_tasks() {
        let tasks = DbTools::get_all_tasks();
        let json = serde_json::to_string_pretty(&tasks).unwrap();

        println!("{}", json)
    }

    #[test]
    fn test_update_finished() {
        // 一条存在的记录
        let id = 2;
        let res = DbTools::update_finished_by_id(&id, &true);

        assert_eq!(res, 0);

        // 一条不存在的记录
        let id2 = 10000000;
        let res2 = DbTools::update_finished_by_id(&id2, &true);
        assert_eq!(res2, -1);
    }

    #[test]
    fn test_delete_record() {
        // 一条存在的记录
        // let id = 2;
        // let res = delete_one_record_by_id(&id);

        // assert_eq!(res, 1);

        // 一条不存在的记录
        let id2 = 1000000;
        let res2 = DbTools::delete_one_record_by_id(&id2);

        assert_eq!(res2, 0)
    }

    #[test]
    fn test_update_record() {
        let update_data = NewTask {
            task_name: &String::from("更新任务3"),
            task_start_time: &String::from("2023.12.12"),
            task_end_time: &String::from("2023.12.16"),
        };

        let id = 2;

        let res = DbTools::update_task_by_id(&id, &update_data);

        assert_eq!(res, 1)
    }
}
