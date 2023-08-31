/*
 * Description: 配置文件
 * Created: 2023-08-31 19:20:57
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use std::collections::HashMap;

use serde::Deserialize;
pub const YUQUE_HOST: &str = "https://www.yuque.com";

pub const REFERER: &str = "https://www.yuque.com/login";
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ConfigKeys {
    GetBooks,
}

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub get_books: String,
}

pub fn get_conf(key: String) {
    let j = "
  {
      \"fingerprint\": \"0xF9BA143B95FF6D82\",
      \"location\": \"Menlo Park, CA\"
  }";

    let foo: Conf = serde_json::from_str(&j).unwrap();
    // age.get('ss')
}
