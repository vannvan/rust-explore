// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// macOS 特定配置，减少 IMK 相关警告
#[cfg(target_os = "macos")]
use std::env;

mod cache;
mod libs;
mod yuque_service;

use libs::models::*;
use std::sync::{Arc, Mutex};
use tauri::State;
use yuque_service::YuqueService;

// 全局语雀服务状态
struct YuqueState(Arc<Mutex<YuqueService>>);

#[tauri::command]
async fn login_yuque(
    account: YuqueAccount,
    state: State<'_, YuqueState>,
) -> Result<LoginResponse, String> {
    let state_clone = state.0.clone();

    // 在异步上下文中获取锁
    let mut service = {
        let state_guard = state_clone.lock().map_err(|_| "Failed to lock service")?;
        state_guard.clone()
    };

    let result = service.login(&account).await;

    // 将更新后的 service 写回状态
    {
        let mut state_guard = state_clone.lock().map_err(|_| "Failed to lock service")?;
        *state_guard = service;
    }

    result.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_user_info(state: State<'_, YuqueState>) -> Result<ApiResponse<YuqueUserInfo>, String> {
    let state_clone = state.0.clone();

    // 在异步上下文中获取锁
    let service = {
        let state_guard = state_clone.lock().map_err(|_| "Failed to lock service")?;
        state_guard.clone()
    };

    let result = service.get_user_info().await;

    result.map_err(|e| e.to_string())
}

#[tauri::command]
async fn check_login_status(state: State<'_, YuqueState>) -> Result<bool, String> {
    let state_clone = state.0.clone();

    // 在异步上下文中获取锁
    let service = {
        let state_guard = state_clone.lock().map_err(|_| "Failed to lock service")?;
        state_guard.clone()
    };

    let result = service.check_login_status().await;

    result.map_err(|e| e.to_string())
}

#[tauri::command]
fn get_cached_user_info(state: State<'_, YuqueState>) -> Result<Option<YuqueUserInfo>, String> {
    let service = state.0.lock().map_err(|_| "Failed to lock service")?;

    Ok(service.get_user_info_sync())
}

#[tauri::command]
fn get_cached_cookies(state: State<'_, YuqueState>) -> Result<Vec<String>, String> {
    let service = state.0.lock().map_err(|_| "Failed to lock service")?;

    Ok(service.get_cookies())
}

#[tauri::command]
fn set_cached_user_info(
    user_info: YuqueUserInfo,
    state: State<'_, YuqueState>,
) -> Result<(), String> {
    let mut service = state.0.lock().map_err(|_| "Failed to lock service")?;

    service.set_user_info(user_info);
    Ok(())
}

#[tauri::command]
fn set_cached_cookies(cookies: Vec<String>, state: State<'_, YuqueState>) -> Result<(), String> {
    let mut service = state.0.lock().map_err(|_| "Failed to lock service")?;
    service.set_cookies(cookies);
    Ok(())
}

#[tauri::command]
fn clear_login_status(state: State<'_, YuqueState>) -> Result<(), String> {
    let mut service = state.0.lock().map_err(|_| "Failed to lock service")?;
    service.clear_login_status();
    Ok(())
}

#[tauri::command]
async fn get_personal_books(state: State<'_, YuqueState>) -> Result<BooksResponse, String> {
    let state_clone = state.0.clone();

    // 克隆 service 以避免在异步上下文中持有锁
    let mut service = {
        let state_guard = state_clone.lock().map_err(|_| "Failed to lock service")?;
        state_guard.clone()
    };

    let result = service.get_personal_books().await;

    // 将更新后的 service 写回状态
    {
        let mut state_guard = state_clone.lock().map_err(|_| "Failed to lock service")?;
        *state_guard = service;
    }

    result.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_team_books(state: State<'_, YuqueState>) -> Result<BooksResponse, String> {
    let state_clone = state.0.clone();

    // 克隆 service 以避免在异步上下文中持有锁
    let mut service = {
        let state_guard = state_clone.lock().map_err(|_| "Failed to lock service")?;
        state_guard.clone()
    };

    let result = service.get_team_books().await;

    // 将更新后的 service 写回状态
    {
        let mut state_guard = state_clone.lock().map_err(|_| "Failed to lock service")?;
        *state_guard = service;
    }

    result.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_book_stacks(state: State<'_, YuqueState>) -> Result<BooksResponse, String> {
    let state_clone = state.0.clone();

    // 克隆 service 以避免在异步上下文中持有锁
    let mut service = {
        let state_guard = state_clone.lock().map_err(|_| "Failed to lock service")?;
        state_guard.clone()
    };

    let result = service.get_book_stacks().await;

    // 将更新后的 service 写回状态
    {
        let mut state_guard = state_clone.lock().map_err(|_| "Failed to lock service")?;
        *state_guard = service;
    }

    result.map_err(|e| e.to_string())
}

#[tauri::command]
async fn expand_window(window: tauri::Window) -> Result<(), String> {
    println!("Debug: 开始展开窗口...");

    // 展开窗口到主程序尺寸
    match window.set_size(tauri::Size::Logical(tauri::LogicalSize::new(1200.0, 800.0))) {
        Ok(_) => println!("Debug: 窗口尺寸设置成功: 1200x800"),
        Err(e) => println!("Debug: 窗口尺寸设置失败: {}", e),
    }

    // 允许调整大小
    match window.set_resizable(true) {
        Ok(_) => println!("Debug: 窗口可调整大小设置成功"),
        Err(e) => println!("Debug: 窗口可调整大小设置失败: {}", e),
    }

    // 设置最小尺寸
    match window.set_min_size(Some(tauri::Size::Logical(tauri::LogicalSize::new(
        800.0, 600.0,
    )))) {
        Ok(_) => println!("Debug: 窗口最小尺寸设置成功: 800x600"),
        Err(e) => println!("Debug: 窗口最小尺寸设置失败: {}", e),
    }

    // 更新窗口标题
    match window.set_title("语雀工具") {
        Ok(_) => println!("Debug: 窗口标题更新成功: 语雀工具"),
        Err(e) => println!("Debug: 窗口标题更新失败: {}", e),
    }

    // 窗口居中显示
    match window.center() {
        Ok(_) => println!("Debug: 窗口居中成功"),
        Err(e) => println!("Debug: 窗口居中失败: {}", e),
    }

    println!("Debug: 窗口展开完成");
    Ok(())
}

#[tauri::command]
async fn clear_books_cache(state: State<'_, YuqueState>) -> Result<(), String> {
    let service = state.0.lock().map_err(|_| "Failed to lock service")?;

    if let Err(e) = service.clear_books_cache() {
        println!("Debug: 清除知识库缓存失败: {}", e);
        return Err(format!("清除知识库缓存失败: {}", e));
    }

    println!("Debug: 知识库缓存清除成功");
    Ok(())
}

#[tauri::command]
async fn clear_docs_cache(state: State<'_, YuqueState>) -> Result<(), String> {
    let service = state.0.lock().map_err(|_| "Failed to lock service")?;

    if let Err(e) = service.clear_docs_cache() {
        println!("Debug: 清除文档缓存失败: {}", e);
        return Err(format!("清除文档缓存失败: {}", e));
    }

    println!("Debug: 文档缓存清除成功");
    Ok(())
}

#[tauri::command]
async fn shrink_window(window: tauri::Window) -> Result<(), String> {
    println!("Debug: 开始收缩窗口...");

    // 收缩窗口到登录页面尺寸
    match window.set_size(tauri::Size::Logical(tauri::LogicalSize::new(400.0, 500.0))) {
        Ok(_) => println!("Debug: 窗口尺寸设置成功: 400x500"),
        Err(e) => println!("Debug: 窗口尺寸设置失败: {}", e),
    }

    // 禁止调整大小
    match window.set_resizable(false) {
        Ok(_) => println!("Debug: 窗口不可调整大小设置成功"),
        Err(e) => println!("Debug: 窗口不可调整大小设置失败: {}", e),
    }

    // 设置固定尺寸
    match window.set_min_size(Some(tauri::Size::Logical(tauri::LogicalSize::new(
        400.0, 500.0,
    )))) {
        Ok(_) => println!("Debug: 窗口最小尺寸设置成功: 400x500"),
        Err(e) => println!("Debug: 窗口最小尺寸设置失败: {}", e),
    }

    match window.set_max_size(Some(tauri::Size::Logical(tauri::LogicalSize::new(
        400.0, 500.0,
    )))) {
        Ok(_) => println!("Debug: 窗口最大尺寸设置成功: 400x500"),
        Err(e) => println!("Debug: 窗口最大尺寸设置失败: {}", e),
    }

    // 更新窗口标题
    match window.set_title("语雀工具 - 登录") {
        Ok(_) => println!("Debug: 窗口标题更新成功: 语雀工具 - 登录"),
        Err(e) => println!("Debug: 窗口标题更新失败: {}", e),
    }

    // 窗口居中显示
    match window.center() {
        Ok(_) => println!("Debug: 窗口居中成功"),
        Err(e) => println!("Debug: 窗口居中失败: {}", e),
    }

    println!("Debug: 窗口收缩完成");
    Ok(())
}

#[tauri::command]
async fn export_document(
    state: State<'_, YuqueState>,
    doc: DocItem,
    book_slug: String, // 添加知识库slug参数
    output_dir: String,
) -> Result<String, String> {
    let service_clone = {
        let service = state.0.lock().map_err(|_| "Failed to lock service")?;
        service.clone()
    };
    // 打印 doc.title, doc.slug, book_slug, output_dir 以便调试
    println!(
        "导出文档: title = {:?}, slug = {:?}, url = {:?}, book_slug = {:?}, output_dir = {:?}",
        doc.title, doc.slug, doc.url, book_slug, output_dir
    );

    // 新增：打印 doc_full_path 字段，确认是否正确传递
    println!("导出文档完整路径: doc_full_path = {:?}", doc.doc_full_path);

    match service_clone
        .export_document(&doc, &book_slug, &output_dir)
        .await
    {
        Ok(file_path) => Ok(file_path),
        Err(e) => Err(format!("导出失败: {}", e)),
    }
}

#[tauri::command]
async fn export_documents(
    state: State<'_, YuqueState>,
    docs: Vec<DocItem>,
    book_slug: String, // 添加知识库slug参数
    output_dir: String,
) -> Result<Vec<String>, String> {
    let service_clone = {
        let service = state.0.lock().map_err(|_| "Failed to lock service")?;
        service.clone()
    };

    // 新增：打印每个文档的 doc_full_path 字段，确认是否正确传递
    println!("批量导出文档信息:");
    for (i, doc) in docs.iter().enumerate() {
        println!(
            "文档 {}: title = {:?}, doc_full_path = {:?}",
            i + 1,
            doc.title,
            doc.doc_full_path
        );
    }

    match service_clone
        .export_documents(&docs, &book_slug, &output_dir)
        .await
    {
        Ok(file_paths) => Ok(file_paths),
        Err(e) => Err(format!("批量导出失败: {}", e)),
    }
}

#[tauri::command]
async fn get_downloads_path() -> Result<String, String> {
    // 获取用户下载目录
    if let Some(downloads_dir) = dirs::download_dir() {
        Ok(downloads_dir.to_string_lossy().to_string())
    } else {
        // 如果无法获取下载目录，使用当前工作目录
        std::env::current_dir()
            .map(|path| path.to_string_lossy().to_string())
            .map_err(|e| format!("Failed to get current directory: {}", e))
    }
}

fn main() {
    println!("Starting Yuque Tools GUI...");

    // macOS 特定配置，减少 IMK 相关警告
    #[cfg(target_os = "macos")]
    {
        // 设置环境变量，减少输入法相关的警告
        env::set_var("NSSupportsAutomaticGraphicsSwitching", "true");

        // 禁用某些 macOS 特定的功能，减少警告
        env::set_var("NSDocumentRevisionsKeepEveryOne", "false");

        println!("macOS 特定配置已应用");
    }

    tauri::Builder::default()
        .manage(YuqueState(Arc::new(Mutex::new(YuqueService::new()))))
        .invoke_handler(tauri::generate_handler![
            login_yuque,
            get_user_info,
            check_login_status,
            get_cached_user_info,
            get_cached_cookies,
            set_cached_user_info,
            set_cached_cookies,
            clear_login_status,
            clear_books_cache,
            clear_docs_cache,
            get_personal_books,
            get_team_books,
            get_book_stacks,
            expand_window,
            shrink_window,
            export_document,
            export_documents,
            get_downloads_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
