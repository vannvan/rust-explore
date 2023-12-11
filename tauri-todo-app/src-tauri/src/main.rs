#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod models;
pub mod schema;
pub mod sql;

use models::NewTask;
use sql::DbTools;

use self::models::Task;

/**
 * 新增
 */
#[tauri::command]
fn add_task(task_name: String, deadline: Vec<String>) -> i32 {
    println!("参数信息: {},{:?}", task_name, deadline);

    let connection = &mut DbTools::establish_connection();

    let new_task = NewTask {
        task_name: &String::from(task_name),
        task_start_time: &deadline[0],
        task_end_time: &deadline[1],
    };
    let post = DbTools::create_task(connection, &new_task);
    post.id
}

/**
 * 获取列表
 */
#[tauri::command]
fn get_tasks() -> Vec<Task> {
    let tasks = DbTools::get_all_tasks();
    tasks
}

/**
 * 删除
 */
#[tauri::command]
fn delete_task(id: i32) -> i32 {
    let res = DbTools::delete_one_record_by_id(&id);
    res
}

/**
 * 更新
 */
#[tauri::command]
fn update_task(id: i32, update_data: NewTask) -> i32 {
    let res = DbTools::update_task_by_id(&id, &update_data);
    res
}

#[tauri::command]
fn finish_task(id: i32, target: bool) -> i32 {
    println!("finish_task 参数:{},{}", id, target);
    let res = DbTools::update_finished_by_id(&id, &target);
    res
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            add_task,
            get_tasks,
            delete_task,
            update_task,
            finish_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
