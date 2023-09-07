/*
 * Description: 全局配置加载
 * Created: 2023-09-03 19:16:02
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fs;

const CONFIG_FILE_PATH: &str = "src/config/config.json";

fn parse_json<T: DeserializeOwned>(schema: &str) -> Option<T> {
    match serde_json::from_str(schema) {
        Ok(parsed) => Some(parsed),
        Err(e) => {
            eprintln!("读取配置文件出错: {}", e.to_string());
            None
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
    pub yuque_host: String,
    pub yuque_referer: String,
    /// 登录接口
    pub yuque_login: String,
    /// 知识库列表
    pub yuque_book_stacks: String,
    /// 某个知识库的信息
    pub yuque_books_info: String,
    /// 导出md文件
    pub yuque_export_markdown: String,
    /// meta
    pub meta_dir: String,
    /// 文档输出目录
    pub target_output_dir: String,
    /// cookies_file
    pub cookies_file: String,
    /// 用户信息
    pub user_info_file: String,
    /// 知识库信息
    pub books_info_file: String,
    /// 过期时间,1天
    pub local_expire: u128,
}

/// 加载配置
pub fn load_conf() -> GlobalConfig {
    let config_file = fs::read_to_string(CONFIG_FILE_PATH).unwrap();
    let parsed_json = parse_json::<GlobalConfig>(&config_file).unwrap();
    parsed_json
}

#[test]

fn test_fn() {
    let config = load_conf();
    println!("{:?}", config)
}
