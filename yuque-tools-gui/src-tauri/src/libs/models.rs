use serde::{Deserialize, Serialize};

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

/// 文档项目结构
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
    pub doc_full_path: Option<String>, // 文档的完整路径，用于构建导出文件的目录结构
}

/// 知识库项目结构
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

/// 知识库列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BooksResponse {
    pub success: bool,
    pub data: Option<Vec<BookItem>>,
    pub message: Option<String>,
    pub total_count: Option<usize>,
}
