use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Encrypt, RsaPublicKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::cache::{CacheManager, CachedBookItem, CachedDocItem, CachedUser};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YuqueAccount {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YuqueUserInfo {
    pub id: i64,
    pub login: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub user_info: Option<YuqueUserInfo>,
    pub cookies: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

// 新增：文档项目结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocItem {
    pub title: String,
    #[serde(rename = "type")]
    pub node_type: String, // "DOC" 或 "TITLE"
    pub uuid: String,
    pub child_uuid: String,
    pub parent_uuid: String,
    pub visible: u8,
    pub url: String,
    pub slug: Option<String>, // 文档的slug，用于构建导出URL
    pub doc_id: Option<String>,
    pub id: Option<String>,
    pub open_window: Option<u8>,
    pub prev_uuid: Option<String>,
    pub sibling_uuid: Option<String>,
    pub level: Option<u8>,
}

// 新增：知识库项目结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookItem {
    pub name: String,
    pub slug: String,
    pub stack_id: Option<String>,
    pub book_id: Option<i64>,
    pub user_login: String,
    pub user_name: String,
    pub book_type: String, // "owner" 或 "collab"
    pub docs: Vec<DocItem>,
}

// 新增：知识库列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BooksResponse {
    pub success: bool,
    pub data: Option<Vec<BookItem>>,
    pub message: Option<String>,
    pub total_count: Option<usize>,
}

#[derive(Clone)]
pub struct YuqueService {
    client: Client,
    cookies: Vec<String>,
    user_info: Option<YuqueUserInfo>,
    cache_manager: CacheManager,
}

impl YuqueService {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .build()
            .unwrap_or_default();

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
                    })
                    .collect(),
            })
            .collect();

        if let Err(e) = self.cache_manager.save_books_info(cached_books) {
            eprintln!("Failed to save books info to cache: {}", e);
        }
    }

    // 构建请求头
    fn build_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();

        // 必需的请求头，参考 yuque-tools 项目
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );
        headers.insert("referer", "https://www.yuque.com".parse().unwrap());
        headers.insert("origin", "https://www.yuque.com".parse().unwrap());
        headers.insert("User-Agent", "Mozilla/5.0 (iPhone; CPU iPhone OS 16_6_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/20G81 YuqueMobileApp/1.0.2 (AppBuild/650 Device/Phone Locale/zh-cn Theme/light YuqueType/public)".parse().unwrap());

        // 额外的请求头
        // headers.insert(
        //     "Accept",
        //     "application/json, text/plain, */*".parse().unwrap(),
        // );
        // headers.insert(
        //     "Accept-Language",
        //     "zh-CN,zh;q=0.9,en;q=0.8".parse().unwrap(),
        // );
        // headers.insert("Accept-Encoding", "gzip, deflate, br".parse().unwrap());
        // headers.insert("Connection", "keep-alive".parse().unwrap());
        // headers.insert("x-requested-with", "XMLHttpRequest".parse().unwrap());

        // 添加 cookies 如果存在
        if !self.cookies.is_empty() {
            // cookies 现在是作为单个完整字符串存储的
            let cookie_header = &self.cookies[0];
            println!("Debug: 使用的cookie头: {}", cookie_header);
            headers.insert("Cookie", cookie_header.parse().unwrap());
        }

        headers
    }

    // 生成当前时间戳
    fn gen_timestamp() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }

    // 加密密码（使用语雀的RSA加密方式）
    fn encrypt_password(&self, password: &str) -> String {
        const RSA_2048_PUB_PEM: &str = include_str!("yuque.pem");
        let pub_key = RsaPublicKey::from_public_key_pem(RSA_2048_PUB_PEM).unwrap();
        let mut rng = rand::thread_rng();

        // 构造密码格式：时间戳:密码
        let password_with_timestamp = [
            Self::gen_timestamp().to_string(),
            ":".to_string(),
            password.to_string(),
        ]
        .join("");

        // 使用RSA公钥加密
        let enc_data = pub_key
            .encrypt(
                &mut rng,
                Pkcs1v15Encrypt,
                password_with_timestamp.as_bytes(),
            )
            .unwrap();

        // 返回base64编码的加密结果
        general_purpose::STANDARD.encode(enc_data)
    }

    // 登录语雀
    pub async fn login(
        &mut self,
        account: &YuqueAccount,
    ) -> Result<LoginResponse, Box<dyn std::error::Error + Send + Sync>> {
        let encrypted_password = self.encrypt_password(&account.password);
        let login_type = "password".to_string();

        let mut params = HashMap::new();
        params.insert("login", &account.username);
        params.insert("password", &encrypted_password);
        params.insert("loginType", &login_type);

        let response = self
            .client
            .post("https://www.yuque.com/api/mobile_app/accounts/login?language=zh-cn")
            .headers(self.build_headers())
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            // 获取状态码和错误信息
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
        // 提取 cookies - 参考 yuque-tools 的实现
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

        println!("Debug: 提取到的完整cookies字符串: {}", cookies_string);

        if !cookies_string.is_empty() {
            // 将完整的cookies字符串保存为单个字符串，这样在请求头中使用时格式正确
            self.cookies = vec![cookies_string];
            println!("Debug: 保存cookies成功，数量: {}", self.cookies.len());
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

                // 保存到缓存
                self.save_login_to_cache(&user_info);

                return Ok(LoginResponse {
                    success: true,
                    message: "登录成功".to_string(),
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
                .unwrap_or("登录失败，请检查用户名和密码")
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
                message: Some("未登录".to_string()),
            });
        }

        let response = self
            .client
            .get("https://www.yuque.com/api/mine")
            .headers(self.build_headers())
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
                message: Some("获取用户信息失败".to_string()),
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
            .get("https://www.yuque.com/api/mine")
            .headers(self.build_headers())
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
        println!("Debug: Setting cookies, count: {}", cookies.len());
        if !cookies.is_empty() {
            println!("Debug: First cookie: {}", cookies[0]);
        }
        self.cookies = cookies;
        println!("Debug: Cookies set, current count: {}", self.cookies.len());
    }

    // 获取个人知识库列表
    pub async fn get_personal_books(
        &mut self,
    ) -> Result<BooksResponse, Box<dyn std::error::Error + Send + Sync>> {
        println!(
            "Debug: 获取个人知识库 - cookies count: {}",
            self.cookies.len()
        );
        if !self.cookies.is_empty() {
            println!("Debug: first cookie: {}", self.cookies[0]);
        }

        if self.cookies.is_empty() {
            return Ok(BooksResponse {
                success: false,
                data: None,
                message: Some("未登录".to_string()),
                total_count: None,
            });
        }

        // 获取个人知识库
        println!("Debug: 开始获取个人知识库...");
        let personal_response = self
            .client
            .get("https://www.yuque.com/api/mine/book_stacks")
            .headers(self.build_headers())
            .send()
            .await?;

        println!("Debug: 个人知识库响应状态: {}", personal_response.status());

        let mut personal_books: Vec<BookItem> = vec![];

        if personal_response.status().is_success() {
            let personal_data: serde_json::Value = personal_response.json().await?;
            println!(
                "Debug: 个人知识库响应数据: {}",
                serde_json::to_string_pretty(&personal_data).unwrap_or_default()
            );

            if let Some(data) = personal_data.get("data") {
                println!(
                    "Debug: 找到 data 字段，类型: {:?}",
                    std::any::type_name_of_val(data)
                );

                // data 是一个数组，每个元素包含一个 books 字段
                if let Some(data_array) = data.as_array() {
                    println!("Debug: data 是数组，长度: {}", data_array.len());

                    for (stack_index, stack) in data_array.iter().enumerate() {
                        println!(
                            "Debug: 处理第 {} 个 stack: {}",
                            stack_index,
                            serde_json::to_string(stack).unwrap_or_default()
                        );

                        if let Some(books) = stack.get("books") {
                            println!(
                                "Debug: stack {} 找到 books 字段，类型: {:?}",
                                stack_index,
                                std::any::type_name_of_val(books)
                            );

                            if let Some(books_array) = books.as_array() {
                                println!(
                                    "Debug: stack {} books 是数组，长度: {}",
                                    stack_index,
                                    books_array.len()
                                );

                                for (book_index, book) in books_array.iter().enumerate() {
                                    println!(
                                        "Debug: 处理 stack {} 第 {} 个知识库: {}",
                                        stack_index,
                                        book_index,
                                        serde_json::to_string(book).unwrap_or_default()
                                    );

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
                                        docs: vec![], // 暂时为空，后续可以获取文档列表
                                    };
                                    personal_books.push(book_item);
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("Debug: 个人知识库总数: {}", personal_books.len());
        for (i, book) in personal_books.iter().enumerate() {
            println!("Debug: 个人知识库 {}: {} ({})", i, book.name, book.slug);
        }

        // 为每个知识库获取文档列表
        let mut total_docs_count = 0;
        let mut cache_hit_count = 0;
        let mut api_fetch_count = 0;

        for book in &mut personal_books {
            println!("Debug: 开始获取知识库 '{}' 的文档列表...", book.name);

            if let Ok(docs) = self.get_book_docs_info(&book.user_login, &book.slug).await {
                book.docs = docs;
                total_docs_count += book.docs.len();

                // 检查是否来自缓存（通过比较获取时间来判断）
                let is_from_cache = self
                    .cache_manager
                    .get_cached_docs(&book.user_login, &book.slug)
                    .is_some();
                if is_from_cache {
                    cache_hit_count += 1;
                    println!(
                        "Debug: [缓存命中] 知识库 '{}' 从缓存获取到 {} 个文档",
                        book.name,
                        book.docs.len()
                    );
                } else {
                    api_fetch_count += 1;
                    println!(
                        "Debug: [接口获取] 知识库 '{}' 从接口获取到 {} 个文档",
                        book.name,
                        book.docs.len()
                    );
                }
            } else {
                println!("Debug: 知识库 '{}' 获取文档失败", book.name);
            }
        }

        println!(
            "Debug: [总结] 个人知识库总数: {}，总文档数: {}，缓存命中: {}，接口获取: {}",
            personal_books.len(),
            total_docs_count,
            cache_hit_count,
            api_fetch_count
        );

        Ok(BooksResponse {
            success: true,
            data: Some(personal_books.clone()),
            message: Some("个人知识库".to_string()),
            total_count: Some(personal_books.len()),
        })
    }

    // 获取团队知识库列表
    pub async fn get_team_books(
        &mut self,
    ) -> Result<BooksResponse, Box<dyn std::error::Error + Send + Sync>> {
        println!(
            "Debug: 获取团队知识库 - cookies count: {}",
            self.cookies.len()
        );
        if !self.cookies.is_empty() {
            println!("Debug: first cookie: {}", self.cookies[0]);
        }

        if self.cookies.is_empty() {
            return Ok(BooksResponse {
                success: false,
                data: None,
                message: Some("未登录".to_string()),
                total_count: None,
            });
        }

        // 获取团队知识库
        println!("Debug: 开始获取团队知识库...");
        let team_response = self
            .client
            .get("https://www.yuque.com/api/mine/user_books?user_type=Group")
            .headers(self.build_headers())
            .send()
            .await?;

        println!("Debug: 团队知识库响应状态: {}", team_response.status());

        let mut team_books: Vec<BookItem> = vec![];

        if team_response.status().is_success() {
            let team_data: serde_json::Value = team_response.json().await?;
            println!(
                "Debug: 团队知识库响应数据: {}",
                serde_json::to_string_pretty(&team_data).unwrap_or_default()
            );

            if let Some(data) = team_data.get("data") {
                println!("Debug: 找到团队 data 字段");
                if let Some(data_array) = data.as_array() {
                    println!("Debug: 团队 data 是数组，长度: {}", data_array.len());
                    for (i, group) in data_array.iter().enumerate() {
                        println!(
                            "Debug: 处理第 {} 个团队: {}",
                            i,
                            serde_json::to_string(group).unwrap_or_default()
                        );
                        if let Some(books) = group.get("books") {
                            println!("Debug: 团队 {} 找到 books 字段", i);
                            if let Some(books_array) = books.as_array() {
                                println!(
                                    "Debug: 团队 {} books 是数组，长度: {}",
                                    i,
                                    books_array.len()
                                );
                                for (j, book) in books_array.iter().enumerate() {
                                    println!(
                                        "Debug: 处理团队 {} 第 {} 个知识库: {}",
                                        i,
                                        j,
                                        serde_json::to_string(book).unwrap_or_default()
                                    );
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
                                        docs: vec![], // 暂时为空，后续可以获取文档列表
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
        let mut total_docs_count = 0;
        let mut cache_hit_count = 0;
        let mut api_fetch_count = 0;

        for book in &mut team_books {
            println!("Debug: 开始获取团队知识库 '{}' 的文档列表...", book.name);

            if let Ok(docs) = self.get_book_docs_info(&book.user_login, &book.slug).await {
                book.docs = docs;
                total_docs_count += book.docs.len();

                // 检查是否来自缓存（通过比较获取时间来判断）
                let is_from_cache = self
                    .cache_manager
                    .get_cached_docs(&book.user_login, &book.slug)
                    .is_some();
                if is_from_cache {
                    cache_hit_count += 1;
                    println!(
                        "Debug: [缓存命中] 团队知识库 '{}' 从缓存获取到 {} 个文档",
                        book.name,
                        book.docs.len()
                    );
                } else {
                    api_fetch_count += 1;
                    println!(
                        "Debug: [接口获取] 团队知识库 '{}' 从接口获取到 {} 个文档",
                        book.name,
                        book.docs.len()
                    );
                }
            } else {
                println!("Debug: 团队知识库 '{}' 获取文档失败", book.name);
            }
        }

        println!(
            "Debug: [总结] 团队知识库总数: {}，总文档数: {}，缓存命中: {}，接口获取: {}",
            team_books.len(),
            total_docs_count,
            cache_hit_count,
            api_fetch_count
        );
        for (i, book) in team_books.iter().enumerate() {
            println!("Debug: 团队知识库 {}: {} ({})", i, book.name, book.slug);
        }

        Ok(BooksResponse {
            success: true,
            data: Some(team_books.clone()),
            message: Some("团队知识库".to_string()),
            total_count: Some(team_books.len()),
        })
    }

    // 获取所有知识库列表（兼容旧接口）
    pub async fn get_book_stacks(
        &mut self,
    ) -> Result<BooksResponse, Box<dyn std::error::Error + Send + Sync>> {
        println!(
            "Debug: 获取所有知识库 - cookies count: {}",
            self.cookies.len()
        );
        if !self.cookies.is_empty() {
            println!("Debug: first cookie: {}", self.cookies[0]);
        }

        if self.cookies.is_empty() {
            return Ok(BooksResponse {
                success: false,
                data: None,
                message: Some("未登录".to_string()),
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

        println!("Debug: 最终获取到的知识库总数: {}", all_books.len());
        for (i, book) in all_books.iter().enumerate() {
            println!(
                "Debug: 知识库 {}: {} ({}) - 类型: {}",
                i, book.name, book.slug, book.book_type
            );
        }

        Ok(BooksResponse {
            success: true,
            data: Some(all_books.clone()),
            message: Some("所有知识库".to_string()),
            total_count: Some(all_books.len()),
        })
    }

    // 获取团队知识库
    pub async fn get_space_books(
        &self,
    ) -> Result<ApiResponse<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
        if self.cookies.is_empty() {
            return Ok(ApiResponse {
                success: false,
                data: None,
                message: Some("未登录".to_string()),
            });
        }

        let response = self
            .client
            .get("https://www.yuque.com/api/mine/user_books?user_type=Group")
            .headers(self.build_headers())
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

        Ok(ApiResponse {
            success: true,
            data: Some(data),
            message: None,
        })
    }

    /// 获取知识库下的文档列表
    pub async fn get_book_docs_info(
        &self,
        user_login: &str,
        book_slug: &str,
    ) -> Result<Vec<DocItem>, Box<dyn std::error::Error + Send + Sync>> {
        println!(
            "Debug: 开始获取知识库文档列表: /{}/{}",
            user_login, book_slug
        );

        // 首先尝试从缓存获取
        if let Some(cached_docs) = self.cache_manager.get_cached_docs(user_login, book_slug) {
            println!(
                "Debug: [缓存命中] 知识库 '{}/{}' 从缓存获取到 {} 个文档",
                user_login,
                book_slug,
                cached_docs.len()
            );

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
                    slug: Some(cached_doc.uuid), // 使用uuid作为slug
                    doc_id: cached_doc.doc_id,
                    id: cached_doc.id,
                    open_window: cached_doc.open_window,
                    prev_uuid: cached_doc.prev_uuid,
                    sibling_uuid: cached_doc.sibling_uuid,
                    level: cached_doc.level,
                })
                .collect();

            return Ok(docs);
        }

        println!(
            "Debug: [接口获取] 知识库 '{}/{}' 缓存未命中，开始从接口获取文档列表",
            user_login, book_slug
        );

        // 构造知识库的 URL
        let book_url = format!("https://www.yuque.com/{}/{}", user_login, book_slug);

        // 获取知识库页面内容
        let response = self
            .client
            .get(&book_url)
            .headers(self.build_headers())
            .send()
            .await?;

        if !response.status().is_success() {
            println!(
                "Debug: [接口获取] 获取知识库页面失败，状态码: {}",
                response.status()
            );
            return Ok(vec![]);
        }

        let html_content = response.text().await?;

        // 使用正则表达式提取文档数据
        // 参考 yuque-tools 的实现，查找 decodeURIComponent 中的 JSON 数据
        let re = regex::Regex::new(r#"decodeURIComponent\("([^"]+)"\)"#).unwrap();

        if let Some(captures) = re.captures(&html_content) {
            if let Some(encoded_data) = captures.get(1) {
                let decoded_data = urlencoding::decode(encoded_data.as_str())
                    .map_err(|e| format!("URL decode failed: {}", e))?;

                // 解析 JSON 数据
                let json_data: serde_json::Value = serde_json::from_str(&decoded_data)?;

                if let Some(book_data) = json_data.get("book") {
                    if let Some(toc_data) = book_data.get("toc") {
                        let docs = self.parse_toc_to_docs(toc_data)?;

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
                                })
                                .collect::<Vec<_>>(),
                        ) {
                            println!(
                                "Debug: [缓存保存] 知识库 '{}/{}' 文档列表缓存保存失败: {}",
                                user_login, book_slug, e
                            );
                        } else {
                            println!(
                                "Debug: [缓存保存] 知识库 '{}/{}' 文档列表已保存到缓存",
                                user_login, book_slug
                            );
                        }

                        return Ok(docs);
                    }
                }
            }
        }

        println!("Debug: [接口获取] 未找到知识库的文档数据");
        Ok(vec![])
    }

    /// 解析目录数据为文档列表
    fn parse_toc_to_docs(
        &self,
        toc_data: &serde_json::Value,
    ) -> Result<Vec<DocItem>, Box<dyn std::error::Error + Send + Sync>> {
        let mut docs = Vec::new();

        if let Some(toc_array) = toc_data.as_array() {
            self.parse_toc_recursive(toc_array, &mut docs, "".to_string());
        }

        println!("Debug: 解析到 {} 个文档", docs.len());
        Ok(docs)
    }

    /// 递归解析目录结构
    fn parse_toc_recursive(
        &self,
        toc_array: &[serde_json::Value],
        docs: &mut Vec<DocItem>,
        _parent_uuid: String, // 添加下划线前缀，因为现在不再使用这个参数
    ) {
        for item in toc_array {
            let node_type = item
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("TITLE")
                .to_string();

            let uuid = item
                .get("uuid")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let title = item
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let visible = item.get("visible").and_then(|v| v.as_u64()).unwrap_or(1) as u8;

            let url = item
                .get("url")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let child_uuid = item
                .get("child_uuid")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            // 提取其他字段
            let doc_id = item
                .get("doc_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let id = item
                .get("id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let open_window = item
                .get("open_window")
                .and_then(|v| v.as_u64())
                .map(|v| v as u8);

            let prev_uuid = item
                .get("prev_uuid")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let sibling_uuid = item
                .get("sibling_uuid")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            // 从 TOC 数据中提取原始的 parent_uuid 和 level
            let original_parent_uuid = item
                .get("parent_uuid")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let original_level = item.get("level").and_then(|v| v.as_u64()).map(|v| v as u8);

            // 调试信息（在移动值之前打印）
            println!(
                "Debug: 解析文档 '{}' - type: {}, uuid: {}, parent_uuid: '{}', level: {:?}",
                title, node_type, uuid, original_parent_uuid, original_level
            );

            // 添加所有节点到文档列表（包括文档和目录）
            let doc_item = DocItem {
                title,
                node_type,
                uuid: uuid.clone(),
                child_uuid,
                parent_uuid: original_parent_uuid, // 使用原始的 parent_uuid
                visible,
                url,
                slug: Some(uuid.clone()), // 使用uuid作为slug的默认值
                doc_id,
                id,
                open_window,
                prev_uuid,
                sibling_uuid,
                level: original_level, // 使用原始的 level 字段
            };

            docs.push(doc_item);

            // 如果有子目录，递归处理
            if let Some(children) = item.get("children") {
                if let Some(children_array) = children.as_array() {
                    self.parse_toc_recursive(children_array, docs, uuid.clone());
                }
            }
        }
    }

    // 清除登录状态
    pub fn clear_login_status(&mut self) {
        self.cookies.clear();
        self.user_info = None;

        // 清除缓存
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
        book_slug: &str, // 添加知识库slug参数
        output_dir: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        println!("=== 开始导出文档 ===");
        println!("文档标题: {}", doc.title);
        println!("文档UUID: {}", doc.uuid);
        println!("父级UUID: {}", doc.parent_uuid);
        println!("文档URL: {}", doc.url);
        println!("文档类型: {}", doc.node_type);
        println!("文档slug: {:?}", doc.slug);
        println!("知识库slug: {}", book_slug);

        // 检查必要参数
        if doc.slug.is_none() {
            println!("错误: 文档缺少slug字段");
            return Err("文档缺少slug字段，无法构建导出URL".into());
        }

        // 构建文档路径 - 使用正确的格式：{知识库的slug}/{文档的slug}
        let doc_slug = doc.slug.as_ref().unwrap();
        // let repos = format!("{}/{}", book_slug, doc_slug);
        let repos = format!("{}/{}", book_slug, doc.url);
        println!("构建的repos参数: {}", repos);

        // 检查repos参数的有效性
        if book_slug.is_empty() {
            println!("警告: 知识库slug为空");
        }
        if doc_slug.is_empty() {
            println!("警告: 文档slug为空");
        }

        // 获取Markdown内容
        println!("开始获取Markdown内容...");
        let content = self.get_markdown_content(&repos).await?;

        if content.is_empty() {
            println!("错误: 获取到的内容为空");
            return Err("获取文档内容失败，非Markdown文件".into());
        }

        println!("成功获取内容，长度: {} 字符", content.len());

        // 创建输出目录
        let full_path = format!("{}/{}", output_dir, doc.title);
        let file_path = format!("{}.md", full_path);
        println!("输出文件路径: {}", file_path);

        // 确保目录存在
        if let Some(parent) = std::path::Path::new(&full_path).parent() {
            std::fs::create_dir_all(parent)?;
            println!("创建目录: {:?}", parent);
        }

        // 写入文件
        std::fs::write(&file_path, content)?;
        println!("文件写入成功: {}", file_path);

        Ok(file_path)
    }

    /// 批量导出文档
    pub async fn export_documents(
        &self,
        docs: &[DocItem],
        book_slug: &str, // 添加知识库slug参数
        output_dir: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut exported_files = Vec::new();

        for doc in docs {
            match self.export_document(doc, book_slug, output_dir).await {
                Ok(file_path) => {
                    exported_files.push(file_path);
                    println!("导出成功: {}", doc.title);
                }
                Err(e) => {
                    println!("导出失败 {}: {}", doc.title, e);
                }
            }
        }

        Ok(exported_files)
    }

    /// 获取文档的Markdown内容
    async fn get_markdown_content(
        &self,
        repos: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // 使用正确的语雀导出API格式
        // 这里的vannvan是当前用户的login，需要从缓存中获取
        let login = self.user_info.as_ref().unwrap().login.clone();
        let url = format!(
            "https://www.yuque.com/{}/{}/markdown?attachment=true&latexcode=false&anchor=false&linebreak=false",
            login,
            repos
        );

        println!("=== 获取Markdown内容 ===");
        println!("完整URL: {}", url);
        println!("repos参数: {}", repos);
        // println!("当前cookies数量: {}", self.cookies.len());

        // 打印cookies内容（脱敏处理）
        if !self.cookies.is_empty() {
            let cookie_preview = if self.cookies[0].len() > 50 {
                // 安全地获取前50个字符，确保不破坏UTF-8边界
                let mut chars: Vec<char> = self.cookies[0].chars().take(50).collect();
                chars.push('…'); // 使用省略号字符
                chars.into_iter().collect::<String>()
            } else {
                self.cookies[0].clone()
            };
            println!("Cookie预览: {}", cookie_preview);
        }

        println!("发送GET请求...");
        let response = self
            .client
            .get(&url)
            .header("Cookie", self.cookies.join("; "))
            .send()
            .await?;

        let status = response.status();
        println!("收到响应，状态码: {}", status);

        if status.is_success() {
            println!("请求成功，开始读取响应内容...");
            let content = response.text().await?;
            println!("响应内容长度: {} 字符", content.len());

            // 打印内容预览（前200字符）- 安全处理UTF-8边界
            let preview = if content.len() > 200 {
                // 安全地获取前200个字符，确保不破坏UTF-8边界
                let mut chars: Vec<char> = content.chars().take(200).collect();
                if content.chars().count() > 200 {
                    chars.push('…'); // 使用省略号字符
                }
                chars.into_iter().collect::<String>()
            } else {
                content.clone()
            };
            println!("内容预览: {}", preview);

            Ok(content)
        } else {
            println!("请求失败，状态码: {}", status);

            // 尝试获取错误响应内容
            // let error_content = response
            //     .text()
            //     .await
            //     .unwrap_or_else(|_| "无法读取错误响应".to_string());
            // println!("错误响应内容: {}", error_content);

            Err(format!("获取文档内容失败，非Markdown文件: {}", status).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_encryption() {
        let service = YuqueService::new();
        let password = "test_password";
        let encrypted = service.encrypt_password(password);

        // 验证加密结果不为空且与原文不同
        assert!(!encrypted.is_empty());
        assert_ne!(encrypted, password);

        // 验证加密结果包含时间戳（应该包含冒号）
        assert!(encrypted.len() > password.len());

        println!("原始密码: {}", password);
        println!("加密结果: {}", encrypted);
    }

    #[test]
    fn test_timestamp_generation() {
        let timestamp1 = YuqueService::gen_timestamp();
        let timestamp2 = YuqueService::gen_timestamp();

        // 验证时间戳是递增的
        assert!(timestamp2 >= timestamp1);

        println!("时间戳1: {}", timestamp1);
        println!("时间戳2: {}", timestamp2);
    }
}
