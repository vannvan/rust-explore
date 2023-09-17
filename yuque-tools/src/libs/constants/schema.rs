/*
 * Description: 用于类型结构体
 * Created: 2023-09-04 11:48:51
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
/// 缓存cookies信息
pub struct LocalCookiesInfo {
    pub expire_time: u128,
    pub cookies: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// 知识库项目
pub struct BookItem {
    pub name: String,
    pub slug: String,
}
#[derive(Serialize, Deserialize, Debug)]
/// 缓存知识库信息
pub struct BookInfo {
    pub expire_time: u128,
    pub books_info: Vec<BookItem>,
}

#[derive(Serialize, Deserialize, Debug)]
/// 用户的CLI配置
pub struct UserCliConfig {
    pub username: String,
    pub password: String,
    pub doc_range: Vec<String>,
    pub skip: bool,
}

#[derive(Serialize, Deserialize, Debug)]
/// 交互信息
pub struct MutualAnswer {
    pub books: Vec<String>,
    /// 是否跳过本地文件
    pub skip: bool,
    /// 是否保留换行标识
    pub line_break: bool,
}
