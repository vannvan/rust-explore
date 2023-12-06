#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn add_task(task_name: String, deadline: Vec<String>) {
    println!("参数信息:{},{:?}", task_name, deadline)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
