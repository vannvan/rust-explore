/*
 * Description: 本地缓存管理器
 * Created: 2024-01-01
 * Author: yuque-tools-gui
 */

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

// 缓存时效：30分钟（毫秒）
const CACHE_EXPIRE_DURATION: u128 = 30 * 60 * 1000;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CachedUserInfo {
    pub expire_time: u128,
    pub user_info: CachedUser,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CachedUser {
    pub id: i64,
    pub login: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CachedCookies {
    pub expire_time: u128,
    pub cookies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CachedBooksInfo {
    pub expire_time: u128,
    pub books_info: Vec<CachedBookItem>,
    pub all_docs_len: usize,
    pub cache_source: CacheSource,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CacheSource {
    Fresh,  // 来自接口最新获取
    Cached, // 来自缓存
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CachedDocsInfo {
    pub expire_time: u128,
    pub docs: Vec<CachedDocItem>,
    pub cache_source: CacheSource,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CachedBookItem {
    pub name: String,
    pub slug: String,
    pub stack_id: Option<String>,
    pub book_id: Option<i64>,
    pub user_login: String,
    pub user_name: String,
    pub book_type: String, // "owner" 或 "collab"
    pub docs: Vec<CachedDocItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CachedDocItem {
    pub title: String,
    #[serde(rename = "type")]
    pub node_type: String, // "DOC" 或 "TITLE"
    pub uuid: String,
    pub child_uuid: String,
    pub parent_uuid: String,
    pub visible: u8,
    pub url: String,
    pub doc_id: Option<String>,
    pub id: Option<String>,
    pub open_window: Option<u8>,
    pub prev_uuid: Option<String>,
    pub sibling_uuid: Option<String>,
    pub level: Option<u8>,
    pub doc_full_path: Option<String>, // 新增：文档的完整路径，用于构建导出文件的目录结构
}

#[derive(Clone)]
pub struct CacheManager {
    cache_dir: PathBuf,
}

impl CacheManager {
    pub fn new() -> Self {
        let cache_dir = Self::get_cache_dir();

        // 确保缓存目录存在
        if let Err(e) = fs::create_dir_all(&cache_dir) {
            eprintln!("Failed to create cache directory: {}", e);
        }

        Self { cache_dir }
    }

    /// 获取缓存目录路径
    fn get_cache_dir() -> PathBuf {
        let mut cache_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        cache_dir.push("yuque-tools-gui");
        cache_dir.push(".meta");
        cache_dir
    }

    /// 生成当前时间戳（毫秒）
    pub fn gen_timestamp() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }

    /// 检查缓存是否过期
    fn is_cache_expired(expire_time: u128) -> bool {
        expire_time < Self::gen_timestamp()
    }

    /// 获取缓存文件路径
    fn get_cache_file_path(&self, filename: &str) -> PathBuf {
        self.cache_dir.join(filename)
    }

    /// 保存用户信息缓存
    pub fn save_user_info(&self, user_info: CachedUser) -> Result<(), Box<dyn std::error::Error>> {
        let cached_user_info = CachedUserInfo {
            expire_time: Self::gen_timestamp() + CACHE_EXPIRE_DURATION,
            user_info,
        };

        let file_path = self.get_cache_file_path("user_info.json");
        let content = serde_json::to_string_pretty(&cached_user_info)?;
        fs::write(file_path, content)?;

        Ok(())
    }

    /// 获取用户信息缓存
    pub fn get_user_info(&self) -> Option<CachedUser> {
        let file_path = self.get_cache_file_path("user_info.json");

        if !file_path.exists() {
            return None;
        }

        match fs::read_to_string(file_path) {
            Ok(content) => {
                match serde_json::from_str::<CachedUserInfo>(&content) {
                    Ok(cached_info) => {
                        if Self::is_cache_expired(cached_info.expire_time) {
                            // 缓存过期，删除文件
                            let _ = fs::remove_file(self.get_cache_file_path("user_info.json"));
                            None
                        } else {
                            Some(cached_info.user_info)
                        }
                    }
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    /// 保存 Cookies 缓存
    pub fn save_cookies(&self, cookies: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let cached_cookies = CachedCookies {
            expire_time: Self::gen_timestamp() + CACHE_EXPIRE_DURATION,
            cookies,
        };

        let file_path = self.get_cache_file_path("cookies.json");
        let content = serde_json::to_string_pretty(&cached_cookies)?;
        fs::write(file_path, content)?;

        Ok(())
    }

    /// 获取 Cookies 缓存
    pub fn get_cookies(&self) -> Option<Vec<String>> {
        let file_path = self.get_cache_file_path("cookies.json");

        if !file_path.exists() {
            return None;
        }

        match fs::read_to_string(file_path) {
            Ok(content) => {
                match serde_json::from_str::<CachedCookies>(&content) {
                    Ok(cached_cookies) => {
                        if Self::is_cache_expired(cached_cookies.expire_time) {
                            // 缓存过期，删除文件
                            let _ = fs::remove_file(self.get_cache_file_path("cookies.json"));
                            None
                        } else {
                            Some(cached_cookies.cookies)
                        }
                    }
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    /// 保存知识库信息缓存
    pub fn save_books_info(
        &self,
        books_info: Vec<CachedBookItem>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let all_docs_len: usize = books_info.iter().map(|book| book.docs.len()).sum();

        let cached_books_info = CachedBooksInfo {
            expire_time: Self::gen_timestamp() + CACHE_EXPIRE_DURATION,
            books_info,
            all_docs_len,
            cache_source: CacheSource::Fresh,
        };

        let file_path = self.get_cache_file_path("books_info.json");
        let content = serde_json::to_string_pretty(&cached_books_info)?;
        fs::write(file_path, content)?;

        Ok(())
    }

    /// 获取知识库信息缓存
    pub fn get_books_info(&self) -> Option<Vec<CachedBookItem>> {
        let file_path = self.get_cache_file_path("books_info.json");

        if !file_path.exists() {
            return None;
        }

        match fs::read_to_string(file_path) {
            Ok(content) => {
                match serde_json::from_str::<CachedBooksInfo>(&content) {
                    Ok(cached_info) => {
                        if Self::is_cache_expired(cached_info.expire_time) {
                            // 缓存过期，删除文件
                            let _ = fs::remove_file(self.get_cache_file_path("books_info.json"));
                            None
                        } else {
                            Some(cached_info.books_info)
                        }
                    }
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    /// 保存文档列表到缓存
    pub fn save_docs_to_cache(
        &self,
        user_login: &str,
        book_slug: &str,
        docs: &[CachedDocItem],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cache_key = format!("{}_{}_docs", user_login, book_slug);
        let cache_file = self.cache_dir.join(format!("{}.json", cache_key));

        let cached_docs = CachedDocsInfo {
            expire_time: Self::gen_timestamp() + CACHE_EXPIRE_DURATION,
            docs: docs.to_vec(),
            cache_source: CacheSource::Fresh,
        };

        let json_data = serde_json::to_string_pretty(&cached_docs)?;
        fs::write(cache_file, json_data)?;

        Ok(())
    }

    /// 获取缓存的文档列表
    pub fn get_cached_docs(&self, user_login: &str, book_slug: &str) -> Option<Vec<CachedDocItem>> {
        let cache_key = format!("{}_{}_docs", user_login, book_slug);
        let cache_file = self.cache_dir.join(format!("{}.json", cache_key));

        if !cache_file.exists() {
            return None;
        }

        match fs::read_to_string(&cache_file) {
            Ok(json_data) => {
                match serde_json::from_str::<CachedDocsInfo>(&json_data) {
                    Ok(cached_docs) => {
                        if Self::gen_timestamp() < cached_docs.expire_time {
                            Some(cached_docs.docs)
                        } else {
                            // 缓存过期，删除文件
                            let _ = fs::remove_file(&cache_file);
                            None
                        }
                    }
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    /// 清除所有缓存
    pub fn clear_all_cache(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cache_files = ["user_info.json", "cookies.json", "books_info.json"];

        for file in &cache_files {
            let file_path = self.get_cache_file_path(file);
            if file_path.exists() {
                fs::remove_file(file_path)?;
            }
        }

        // 清除文档缓存文件
        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    if let Some(name) = file_name.to_str() {
                        if name.ends_with("_docs.json") {
                            let _ = fs::remove_file(entry.path());
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// 清除用户相关缓存（保留应用配置）
    pub fn clear_user_cache(&self) -> Result<(), Box<dyn std::error::Error>> {
        let user_files = ["user_info.json", "cookies.json", "books_info.json"];

        for file in &user_files {
            let file_path = self.get_cache_file_path(file);
            if file_path.exists() {
                fs::remove_file(file_path)?;
            }
        }

        Ok(())
    }

    /// 清除知识库缓存
    pub fn clear_books_cache(&self) -> Result<(), Box<dyn std::error::Error>> {
        let books_file = self.get_cache_file_path("books_info.json");
        if books_file.exists() {
            fs::remove_file(books_file)?;
            println!("Debug: 知识库缓存文件已删除");
        }
        Ok(())
    }

    /// 清除文档缓存
    pub fn clear_docs_cache(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 清除所有文档缓存文件
        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    if let Some(name) = file_name.to_str() {
                        if name.ends_with("_docs.json") {
                            if let Err(e) = fs::remove_file(entry.path()) {
                                println!("Debug: 删除文档缓存文件失败: {} - {}", name, e);
                            } else {
                                println!("Debug: 文档缓存文件已删除: {}", name);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// 获取缓存统计信息
    pub fn get_cache_stats(&self) -> CacheStats {
        let user_info_cached = self.get_cache_file_path("user_info.json").exists();
        let cookies_cached = self.get_cache_file_path("cookies.json").exists();
        let books_cached = self.get_cache_file_path("books_info.json").exists();

        CacheStats {
            user_info_cached,
            cookies_cached,
            books_cached,
            cache_dir: self.cache_dir.clone(),
        }
    }
}

#[derive(Debug)]
pub struct CacheStats {
    pub user_info_cached: bool,
    pub cookies_cached: bool,
    pub books_cached: bool,
    pub cache_dir: PathBuf,
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_manager_creation() {
        let manager = CacheManager::new();
        assert!(manager.cache_dir.exists());
    }

    #[test]
    fn test_timestamp_generation() {
        let timestamp1 = CacheManager::gen_timestamp();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let timestamp2 = CacheManager::gen_timestamp();
        assert!(timestamp2 > timestamp1);
    }

    #[test]
    fn test_cache_expiration() {
        let current_time = CacheManager::gen_timestamp();
        let expired_time = current_time - 1000; // 1秒前
        let valid_time = current_time + 1000; // 1秒后

        assert!(CacheManager::is_cache_expired(expired_time));
        assert!(!CacheManager::is_cache_expired(valid_time));
    }
}
