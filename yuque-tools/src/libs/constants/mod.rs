/*
 * Description: 配置文件
 * Created: 2023-08-31 19:20:57
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use std::collections::HashMap;

use serde::{Deserialize, Serializer};
use serde_json::Value;
pub const YUQUE_HOST: &str = "https://www.yuque.com";

pub const REFERER: &str = "https://www.yuque.com/login";
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ConfigKeys {
    GetBooks,
}

#[derive(Deserialize, Debug, Serializer)]
pub struct Conf {
    pub get_books: String,
}

fn get_config_value(key: &str) -> Option<&Value> {
    let config_json = r#"
        {
            "get_books": "value1"
        }
    "#;

    let config: HashMap<String, Value> = serde_json::from_str(config_json).unwrap();
    config.get(key)
}

#[test]
fn test() {
    let key = Conf {
        get_books: String::new(),
    }
    .get_books;
    let value = get_config_value(&key);
    println!("{:?}", value);
}
