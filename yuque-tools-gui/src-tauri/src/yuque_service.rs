use reqwest::Client;
use std::collections::HashMap;

use crate::cache::{CacheManager, CachedBookItem, CachedDocItem, CachedUser};
use crate::libs::{
    api_config::{Auth, Books, User},
    constants::{ErrorMessages, SuccessMessages},
    crypto::CryptoUtils,
    doc_parser::DocParser,
    export_utils::ExportUtils,
    http_utils::HttpUtils,
    models::*,
};

/// 语雀服务主类
#[derive(Clone)]
pub struct YuqueService {
    client: Client,
    cookies: Vec<String>,
    user_info: Option<YuqueUserInfo>,
    cache_manager: CacheManager,
}

impl YuqueService {
    pub fn new() -> Self {
        let client = HttpUtils::create_client();
        let cache_manager = CacheManager::new();

        let mut service = Self {
            client,
            cookies: Vec::new(),
            user_info: None,
            cache_manager,
        };

        // 启动时尝试从缓存加载数据
        service.load_from_cache();
        service
    }

    /// 从缓存加载数据
    fn load_from_cache(&mut self) {
        // 加载用户信息
        if let Some(cached_user) = self.cache_manager.get_user_info() {
            self.user_info = Some(YuqueUserInfo {
                id: cached_user.id,
                login: cached_user.login,
                name: cached_user.name,
                avatar_url: cached_user.avatar_url,
                description: cached_user.description,
            });
        }

        // 加载 cookies
        if let Some(cached_cookies) = self.cache_manager.get_cookies() {
            self.cookies = cached_cookies;
        }
    }

    /// 保存登录信息到缓存
    fn save_login_to_cache(&self, user_info: &YuqueUserInfo) {
        // 保存用户信息
        let cached_user = CachedUser {
            id: user_info.id,
            login: user_info.login.clone(),
            name: user_info.name.clone(),
            avatar_url: user_info.avatar_url.clone(),
            description: user_info.description.clone(),
        };

        if let Err(e) = self.cache_manager.save_user_info(cached_user) {
            eprintln!("Failed to save user info to cache: {}", e);
        }

        // 保存 cookies
        if !self.cookies.is_empty() {
            if let Err(e) = self.cache_manager.save_cookies(self.cookies.clone()) {
                eprintln!("Failed to save cookies to cache: {}", e);
            }
        }
    }

    /// 保存知识库信息到缓存
    fn save_books_to_cache(&self, books: &[BookItem]) {
        let cached_books: Vec<CachedBookItem> = books
            .iter()
            .map(|book| CachedBookItem {
                name: book.name.clone(),
                slug: book.slug.clone(),
                stack_id: book.stack_id.clone(),
                book_id: book.book_id,
                user_login: book.user_login.clone(),
                user_name: book.user_name.clone(),
                book_type: book.book_type.clone(),
                docs: book
                    .docs
                    .iter()
                    .map(|doc| CachedDocItem {
                        title: doc.title.clone(),
                        node_type: doc.node_type.clone(),
                        uuid: doc.uuid.clone(),
                        child_uuid: doc.child_uuid.clone(),
                        parent_uuid: doc.parent_uuid.clone(),
                        visible: doc.visible,
                        url: doc.url.clone(),
                        doc_id: doc.doc_id.clone(),
                        id: doc.id.clone(),
                        open_window: doc.open_window,
                        prev_uuid: doc.prev_uuid.clone(),
                        sibling_uuid: doc.sibling_uuid.clone(),
                        level: doc.level,
                        doc_full_path: doc.doc_full_path.clone(),
                    })
                    .collect(),
            })
            .collect();

        if let Err(e) = self.cache_manager.save_books_info(cached_books) {
            eprintln!("Failed to save books info to cache: {}", e);
        }
    }

    // 登录语雀
    pub async fn login(
        &mut self,
        account: &YuqueAccount,
    ) -> Result<LoginResponse, Box<dyn std::error::Error + Send + Sync>> {
        let encrypted_password = CryptoUtils::encrypt_password(&account.password);
        let login_type = "password".to_string();

        let mut params = HashMap::new();
        params.insert("login", &account.username);
        params.insert("password", &encrypted_password);
        params.insert("loginType", &login_type);

        let response = self
            .client
            .post(&Auth::login_url())
            .headers(HttpUtils::build_headers(&self.cookies))
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let err_text = response
                .text()
                .await
                .unwrap_or_else(|_| "无法获取错误信息".to_string());
            println!("HTTP错误: {} - {}", status, err_text);

            return Ok(LoginResponse {
                success: false,
                message: format!("HTTP error: {}: {}", status, err_text),
                user_info: None,
                cookies: None,
            });
        }

        // 提取 cookies
        let cookies_string: String = response
            .headers()
            .iter()
            .filter_map(|(key, value)| {
                if key.to_string().to_lowercase() == "set-cookie" {
                    value.to_str().ok()
                } else {
                    None
                }
            })
            .collect::<Vec<&str>>()
            .join(";");

        if !cookies_string.is_empty() {
            self.cookies = vec![cookies_string];
        }

        let data: serde_json::Value = response.json().await?;

        if let Some(data_obj) = data.get("data") {
            if let Some(me) = data_obj.get("me") {
                let user_info = YuqueUserInfo {
                    id: me.get("id").and_then(|v| v.as_i64()).unwrap_or(0),
                    login: me
                        .get("login")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    name: me
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    avatar_url: me
                        .get("avatar_url")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    description: me
                        .get("description")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                };

                self.user_info = Some(user_info.clone());
                self.save_login_to_cache(&user_info);

                return Ok(LoginResponse {
                    success: true,
                    message: SuccessMessages::LOGIN_SUCCESS.to_string(),
                    user_info: Some(user_info),
                    cookies: Some(self.cookies.clone()),
                });
            }
        }

        Ok(LoginResponse {
            success: false,
            message: data
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or(ErrorMessages::LOGIN_FAILED)
                .to_string(),
            user_info: None,
            cookies: None,
        })
    }

    // 获取用户信息
    pub async fn get_user_info(
        &self,
    ) -> Result<ApiResponse<YuqueUserInfo>, Box<dyn std::error::Error + Send + Sync>> {
        if self.cookies.is_empty() {
            return Ok(ApiResponse {
                success: false,
                data: None,
                message: Some(ErrorMessages::NOT_LOGGED_IN.to_string()),
            });
        }

        let response = self
            .client
            .get(&User::mine_url())
            .headers(HttpUtils::build_headers(&self.cookies))
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(ApiResponse {
                success: false,
                data: None,
                message: Some(format!("HTTP error: {}", response.status())),
            });
        }

        let data: serde_json::Value = response.json().await?;

        if let Some(data_obj) = data.get("data") {
            let user_info = YuqueUserInfo {
                id: data_obj.get("id").and_then(|v| v.as_i64()).unwrap_or(0),
                login: data_obj
                    .get("login")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                name: data_obj
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                avatar_url: data_obj
                    .get("avatar_url")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                description: data_obj
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
            };

            Ok(ApiResponse {
                success: true,
                data: Some(user_info),
                message: None,
            })
        } else {
            Ok(ApiResponse {
                success: false,
                data: None,
                message: Some(ErrorMessages::GET_USER_INFO_FAILED.to_string()),
            })
        }
    }

    // 检查登录状态
    pub async fn check_login_status(
        &self,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if self.cookies.is_empty() {
            return Ok(false);
        }

        let response = self
            .client
            .get(&User::mine_url())
            .headers(HttpUtils::build_headers(&self.cookies))
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    // 获取用户信息（同步版本）
    pub fn get_user_info_sync(&self) -> Option<YuqueUserInfo> {
        self.user_info.clone()
    }

    // 获取 cookies
    pub fn get_cookies(&self) -> Vec<String> {
        self.cookies.clone()
    }

    // 设置用户信息
    pub fn set_user_info(&mut self, user_info: YuqueUserInfo) {
        self.user_info = Some(user_info);
    }

    // 设置 cookies
    pub fn set_cookies(&mut self, cookies: Vec<String>) {
        self.cookies = cookies;
    }

    // 获取个人知识库列表
    pub async fn get_personal_books(
        &mut self,
    ) -> Result<BooksResponse, Box<dyn std::error::Error + Send + Sync>> {
        if self.cookies.is_empty() {
            return Ok(BooksResponse {
                success: false,
                data: None,
                message: Some(ErrorMessages::NOT_LOGGED_IN.to_string()),
                total_count: None,
            });
        }

        let response = self
            .client
            .get(&Books::personal_books_url())
            .headers(HttpUtils::build_headers(&self.cookies))
            .send()
            .await?;

        let mut personal_books: Vec<BookItem> = vec![];

        if response.status().is_success() {
            let personal_data: serde_json::Value = response.json().await?;

            if let Some(data) = personal_data.get("data") {
                if let Some(data_array) = data.as_array() {
                    for stack in data_array.iter() {
                        if let Some(books) = stack.get("books") {
                            if let Some(books_array) = books.as_array() {
                                for book in books_array.iter() {
                                    let book_item = BookItem {
                                        name: book
                                            .get("name")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string(),
                                        slug: book
                                            .get("slug")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string(),
                                        stack_id: book
                                            .get("stack_id")
                                            .and_then(|v| v.as_str())
                                            .map(|s| s.to_string()),
                                        book_id: book.get("id").and_then(|v| v.as_i64()),
                                        user_login: book
                                            .get("user")
                                            .and_then(|u| u.get("login"))
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string(),
                                        user_name: book
                                            .get("user")
                                            .and_then(|u| u.get("name"))
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string(),
                                        book_type: "owner".to_string(),
                                        docs: vec![],
                                    };
                                    personal_books.push(book_item);
                                }
                            }
                        }
                    }
                }
            }
        }

        // 为每个知识库获取文档列表
        for book in &mut personal_books {
            if let Ok(docs) = self.get_book_docs_info(&book.user_login, &book.slug).await {
                book.docs = docs;
            }
        }

        Ok(BooksResponse {
            success: true,
            data: Some(personal_books.clone()),
            message: Some(SuccessMessages::PERSONAL_BOOKS.to_string()),
            total_count: Some(personal_books.len()),
        })
    }

    // 获取团队知识库列表
    pub async fn get_team_books(
        &mut self,
    ) -> Result<BooksResponse, Box<dyn std::error::Error + Send + Sync>> {
        if self.cookies.is_empty() {
            return Ok(BooksResponse {
                success: false,
                data: None,
                message: Some(ErrorMessages::NOT_LOGGED_IN.to_string()),
                total_count: None,
            });
        }

        let response = self
            .client
            .get(&Books::team_books_url())
            .headers(HttpUtils::build_headers(&self.cookies))
            .send()
            .await?;

        let mut team_books: Vec<BookItem> = vec![];

        if response.status().is_success() {
            let team_data: serde_json::Value = response.json().await?;

            if let Some(data) = team_data.get("data") {
                if let Some(data_array) = data.as_array() {
                    for group in data_array.iter() {
                        if let Some(books) = group.get("books") {
                            if let Some(books_array) = books.as_array() {
                                for book in books_array.iter() {
                                    let book_item = BookItem {
                                        name: book
                                            .get("name")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string(),
                                        slug: book
                                            .get("slug")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string(),
                                        stack_id: book
                                            .get("stack_id")
                                            .and_then(|v| v.as_str())
                                            .map(|s| s.to_string()),
                                        book_id: book.get("id").and_then(|v| v.as_i64()),
                                        user_login: book
                                            .get("user")
                                            .and_then(|u| u.get("login"))
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string(),
                                        user_name: book
                                            .get("user")
                                            .and_then(|u| u.get("name"))
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string(),
                                        book_type: "collab".to_string(),
                                        docs: vec![],
                                    };
                                    team_books.push(book_item);
                                }
                            }
                        }
                    }
                }
            }
        }

        // 为每个团队知识库获取文档列表
        for book in &mut team_books {
            if let Ok(docs) = self.get_book_docs_info(&book.user_login, &book.slug).await {
                book.docs = docs;
            }
        }

        Ok(BooksResponse {
            success: true,
            data: Some(team_books.clone()),
            message: Some(SuccessMessages::TEAM_BOOKS.to_string()),
            total_count: Some(team_books.len()),
        })
    }

    // 获取所有知识库列表
    pub async fn get_book_stacks(
        &mut self,
    ) -> Result<BooksResponse, Box<dyn std::error::Error + Send + Sync>> {
        if self.cookies.is_empty() {
            return Ok(BooksResponse {
                success: false,
                data: None,
                message: Some(ErrorMessages::NOT_LOGGED_IN.to_string()),
                total_count: None,
            });
        }

        // 获取个人知识库
        let personal_result = self.get_personal_books().await;
        let personal_books = if let Ok(personal_response) = personal_result {
            if personal_response.success {
                personal_response.data.unwrap_or_default()
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        // 获取团队知识库
        let team_result = self.get_team_books().await;
        let team_books = if let Ok(team_response) = team_result {
            if team_response.success {
                team_response.data.unwrap_or_default()
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        // 合并所有知识库
        let mut all_books = personal_books;
        all_books.extend(team_books);

        // 保存到缓存
        self.save_books_to_cache(&all_books);

        Ok(BooksResponse {
            success: true,
            data: Some(all_books.clone()),
            message: Some(SuccessMessages::ALL_BOOKS.to_string()),
            total_count: Some(all_books.len()),
        })
    }

    /// 获取知识库下的文档列表
    pub async fn get_book_docs_info(
        &self,
        user_login: &str,
        book_slug: &str,
    ) -> Result<Vec<DocItem>, Box<dyn std::error::Error + Send + Sync>> {
        // 首先尝试从缓存获取
        if let Some(cached_docs) = self.cache_manager.get_cached_docs(user_login, book_slug) {
            // 转换为 DocItem 格式
            let docs: Vec<DocItem> = cached_docs
                .into_iter()
                .map(|cached_doc| DocItem {
                    title: cached_doc.title,
                    node_type: cached_doc.node_type,
                    uuid: cached_doc.uuid.clone(),
                    child_uuid: cached_doc.child_uuid,
                    parent_uuid: cached_doc.parent_uuid,
                    visible: cached_doc.visible,
                    url: cached_doc.url,
                    slug: Some(cached_doc.uuid),
                    doc_id: cached_doc.doc_id,
                    id: cached_doc.id,
                    open_window: cached_doc.open_window,
                    prev_uuid: cached_doc.prev_uuid,
                    sibling_uuid: cached_doc.sibling_uuid,
                    level: cached_doc.level,
                    doc_full_path: cached_doc.doc_full_path,
                })
                .collect();

            return Ok(docs);
        }

        // 构造知识库的 URL
        let book_url = Books::book_page_url(user_login, book_slug);

        // 获取知识库页面内容
        let response = self
            .client
            .get(&book_url)
            .headers(HttpUtils::build_headers(&self.cookies))
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(vec![]);
        }

        let html_content = response.text().await?;

        // 使用文档解析器提取文档数据
        let docs = DocParser::extract_docs_from_html(&html_content)?;

        if !docs.is_empty() {
            // 保存到缓存
            if let Err(e) = self.cache_manager.save_docs_to_cache(
                user_login,
                book_slug,
                &docs
                    .iter()
                    .map(|doc| crate::cache::CachedDocItem {
                        title: doc.title.clone(),
                        node_type: doc.node_type.clone(),
                        uuid: doc.uuid.clone(),
                        child_uuid: doc.child_uuid.clone(),
                        parent_uuid: doc.parent_uuid.clone(),
                        visible: doc.visible,
                        url: doc.url.clone(),
                        doc_id: doc.doc_id.clone(),
                        id: doc.id.clone(),
                        open_window: doc.open_window,
                        prev_uuid: doc.prev_uuid.clone(),
                        sibling_uuid: doc.sibling_uuid.clone(),
                        level: doc.level,
                        doc_full_path: doc.doc_full_path.clone(),
                    })
                    .collect::<Vec<_>>(),
            ) {
                println!(
                    "Debug: [缓存保存] 知识库 '{}/{}' 文档列表缓存保存失败: {}",
                    user_login, book_slug, e
                );
            }
        }

        Ok(docs)
    }

    // 清除登录状态
    pub fn clear_login_status(&mut self) {
        self.cookies.clear();
        self.user_info = None;

        if let Err(e) = self.cache_manager.clear_user_cache() {
            eprintln!("Failed to clear cache: {}", e);
        }
    }

    // 清除知识库缓存
    pub fn clear_books_cache(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.cache_manager.clear_books_cache().map_err(|e| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?)
    }

    // 清除文档缓存
    pub fn clear_docs_cache(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.cache_manager.clear_docs_cache().map_err(|e| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?)
    }

    /// 导出单个文档
    pub async fn export_document(
        &self,
        doc: &DocItem,
        book_slug: &str,
        output_dir: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let user_login = self.user_info.as_ref().ok_or("用户未登录")?.login.clone();

        ExportUtils::export_document(
            &self.client,
            doc,
            book_slug,
            output_dir,
            &self.cookies,
            &user_login,
        )
        .await
    }

    /// 批量导出文档
    pub async fn export_documents(
        &self,
        docs: &[DocItem],
        book_slug: &str,
        output_dir: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let user_login = self.user_info.as_ref().ok_or("用户未登录")?.login.clone();

        ExportUtils::export_documents(
            &self.client,
            docs,
            book_slug,
            output_dir,
            &self.cookies,
            &user_login,
        )
        .await
    }
}
