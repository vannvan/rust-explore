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

/// yuque账号信息
pub struct YuqueAccount {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// 用户的CLI配置
pub struct UserCliConfig {
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    /// 表示可以忽略校验的字段，否则会报错
    #[serde(default)]
    pub toc_range: Vec<String>,
    #[serde(default = "default_as_true")]
    pub skip: bool,
    #[serde(default = "default_as_true")]
    pub line_break: bool,
    #[serde(default)]
    /// 自定义域名，只有团队知识库才需要
    pub host: String,
    #[serde(default)]
    /// 自定义输出目录
    pub output: String,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    #[serde(rename = "type")]
    pub node_type: String,
    pub children: Vec<TreeNone>,
    pub title: String,
    pub name: String,
    /// 子文档会有child_uuid
    pub child_uuid: String,
    pub visible: u8,
    /// 父级slug
    pub p_slug: String,
    /// 文档所属user
    pub user: String,
    /// 文档地址
    pub url: String,
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
        #[serde(default)]
        pub child_uuid: String,
        pub parent_uuid: String,
        pub visible: u8,
        /// 这个是必须的，导出的时候需要
        pub url: String,
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

/// 资源列表信息
#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceItem {
    // pub link: String,
    /// 本地保存的完成路径名称
    pub target_save_full_path_name: String,
    /// 资源id
    pub source_id: String,
}
