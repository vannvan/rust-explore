/*
 * Description: 配置文件
 * Created: 2023-08-31 19:20:57
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

pub mod init_load_config;
use std::fs;

use lazy_static::lazy_static;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub const YUQUE_HOST: &str = "https://www.yuque.com";

pub const REFERER: &str = "https://www.yuque.com/login";

const CONFIG_FILE_PATH: &str = "src/libs/constants/config.json";

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
    pub login_api: String,
    pub get_books: String,
}

fn load_conf() -> GlobalConfig {
    let config_file = fs::read_to_string(CONFIG_FILE_PATH).unwrap();
    let parsed_json = parse_json::<GlobalConfig>(&config_file).unwrap();
    parsed_json
}

lazy_static! {
    pub static ref GLOBAL_CONFIG: GlobalConfig = load_conf();
}

#[test]

fn test_fn() {
    let config = load_conf();
    println!("{:?}", config)
}
