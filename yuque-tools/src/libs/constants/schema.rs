/*
 * Description: 用于类型结构体
 * Created: 2023-09-04 11:48:51
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use serde::{Deserialize, Serialize};

fn default_as_true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
/// 缓存cookies信息
pub struct LocalCookiesInfo {
    pub expire_time: u128,
    pub cookies: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// 用户的CLI配置
pub struct UserCliConfig {
    pub username: String,
    pub password: String,
    /// 表示可以忽略校验的字段，否则会报错
    #[serde(default)]
    pub toc_range: Vec<String>,
    #[serde(default = "default_as_true")]
    pub skip: bool,
    #[serde(default = "default_as_true")]
    pub line_break: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YuqueLoginUserInfo {
    pub name: String,
    pub login: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LocalCacheUserInfo {
    pub expire_time: u128,
    pub user_info: YuqueLoginUserInfo,
}

#[derive(Serialize, Deserialize, Debug)]
/// 交互信息
pub struct MutualAnswer {
    /// 知识库范围
    pub toc_range: Vec<String>,
    /// 是否跳过本地文件
    pub skip: bool,
    /// 是否保留换行标识
    pub line_break: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
/// 树形节点
pub struct TreeNone {
    pub parent_id: String,
    pub uuid: String,
    pub full_path: String,
    pub p_slug: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub children: Vec<TreeNone>,
    pub title: String,
    pub name: String,
    pub user: String,
    /// 子文档会有child_uuid
    pub child_uuid: String,
    pub visible: u8,
}

/// 知识库缓存信息
pub mod cache_book {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug)]
    /// 文档项目
    pub struct DocItem {
        pub title: String,
        #[serde(rename = "type")]
        /// 用于区分是目录还是文档 DOC TITLE
        pub node_type: String,
        pub uuid: String,
        pub child_uuid: String,
        pub parent_uuid: String,
        pub visible: u8,
    }
    #[derive(Serialize, Deserialize, Debug)]
    /// 知识库项目
    pub struct BookItem {
        pub name: String,
        pub slug: String,
        pub docs: Vec<DocItem>,
        pub user_login: String,
        /// 用于区分是私有还是公共的
        pub book_type: String,
    }
    #[derive(Serialize, Deserialize, Debug)]
    /// 缓存知识库信息
    pub struct BookInfo {
        pub expire_time: u128,
        pub books_info: Vec<BookItem>,
    }
}
