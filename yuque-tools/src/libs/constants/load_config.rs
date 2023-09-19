/*
 * Description: 全局配置加载
 * Created: 2023-09-03 19:16:02
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
// use std::fs;

// const CONFIG_FILE_PATH: &str = "conf/config.json";

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
    /// meta目录
    pub meta_dir: String,
    /// 用户的CLI配置
    pub user_cli_config_file: String,
    /// 文档输出目录
    pub target_output_dir: String,
    /// cookies_file
    pub cookies_file: String,
    /// 缓存的用户信息
    pub user_info_file: String,
    /// 知识库信息
    pub books_info_file: String,
    /// 过期时间,1天
    pub local_expire: u128,
}
#[derive(Serialize, Deserialize, Debug)]
struct Conf {
    pub host: String,
}

/// 加载配置
pub fn load_conf() -> GlobalConfig {
    // let config_file = fs::read_to_string(CONFIG_FILE_PATH).unwrap();
    let config_file = json!({
        "yuque_host": "https://www.yuque.com",
        "yuque_referer": "https://www.yuque.com/login",
        "yuque_login": "/api/accounts/login",
        "yuque_book_stacks": "/api/mine/book_stacks",
        "yuque_books_info": "",
        "yuque_export_markdown": "",
        "meta_dir": ".meta",
        "target_output_dir": "./docs",
        "user_cli_config_file": "yuque.config.json",
        "cookies_file": ".meta/cookies.json",
        "user_info_file": ".meta/user_info.json",
        "books_info_file": ".meta/books_info.json",
        "duration": 500,
        "local_expire": 86400000
    })
    .to_string();

    // let conf = GlobalConfig {
    //     yuque_host: "https://www.yuque.com",
    //     yuque_referer: "https://www.yuque.com/login",
    //     yuque_login: "/api/accounts/login",
    //     yuque_book_stacks: "/api/mine/book_stacks",
    //     yuque_books_info: "",
    //     yuque_export_markdown: "",
    //     meta_dir: "",
    //     user_cli_config_file: "yuque.config.json",
    //     target_output_dir: "./docs",
    //     cookies_file: ".meta/cookies.json",
    //     user_info_file: "./meta/user_info.json",
    //     books_info_file: ".meta/books_info.json",
    //     local_expire: 86400000,
    // };

    // let parsed_json =
    //     parse_json::<GlobalConfig>(&serde_json::to_string(&conf).unwrap().clone()).unwrap();
    let parsed_json = parse_json::<GlobalConfig>(&config_file.to_string()).unwrap();
    parsed_json
    // conf
}

#[test]

fn test_fn() {
    let config = load_conf();
    println!("{:?}", config)
}
