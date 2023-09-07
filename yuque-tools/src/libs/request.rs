/*
 * Description: 请求
 * Created: 2023-08-31 18:47:09
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

// use config::Config;
use regex::Regex;
use reqwest::header::HeaderMap;

use serde_json::{json, Value};
use std::{collections::HashMap, process};

use crate::libs::{
    constants::GLOBAL_CONFIG,
    file::File,
    log::Log,
    tools::{gen_timestamp, get_local_cookies},
};

#[allow(dead_code)]
pub fn crawl() {
    //
}

pub struct Request {
    pub host: String,
}

impl Request {
    fn request_header() -> HeaderMap {
        // 组装header
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("referer", GLOBAL_CONFIG.yuque_referer.parse().unwrap());
        headers.insert("origin", GLOBAL_CONFIG.yuque_host.parse().unwrap());
        return headers;
    }

    pub async fn get(url: &str) -> Result<HashMap<String, Value>, reqwest::Error> {
        let target_url = GLOBAL_CONFIG.yuque_host.clone() + &url;
        println!("get请求,{}", &target_url);
        // let res = reqwest::get(&target_url);
        // Ok(res.json::<HashMap<String, String>>().await?)
        let client = reqwest::Client::new();

        let res = client
            .get(target_url)
            .header("cookie", get_local_cookies())
            .header("content-type", "application/json")
            .header("x-requested-with", "XMLHttpRequest")
            .send()
            .await?;

        // println!("----{}", res.text().await?);
        Ok(res.json::<HashMap<String, Value>>().await?)
    }

    pub async fn post(
        url: &str,
        params: HashMap<&str, String>,
    ) -> Result<HashMap<String, Value>, reqwest::Error> {
        let client = reqwest::Client::new();
        let header = Self::request_header();

        let target_url = GLOBAL_CONFIG.yuque_host.clone() + &url;
        let res = client
            .post(target_url)
            .headers(header)
            .json(&params)
            .send()
            .await?;

        let login_reg = Regex::new("login");

        // 如果是登录，就存下cookies
        if login_reg.unwrap().is_match(url) {
            let mut vec = vec![];
            for item in res
                .headers()
                .iter()
                .filter(|x| x.0 == "set-cookie")
                .map(|s| s.1.to_str())
            {
                vec.push(item.unwrap())
            }

            let cookies = vec.join(";");

            let cookies_info = json!( {
                "expire_time": gen_timestamp() + GLOBAL_CONFIG.local_expire,
                "cookies": cookies
            });

            let f = File::new();

            match f.mkdir(&GLOBAL_CONFIG.meta_dir) {
                Ok(_) => match f.write(&GLOBAL_CONFIG.cookies_file, cookies_info.to_string()) {
                    Err(_) => {
                        Log::error("文件创建失败");
                        process::exit(1)
                    }
                    Ok(_) => (),
                },
                Err(_) => process::exit(1),
            }
        }

        Ok(res.json::<HashMap<String, Value>>().await?)
    }
}
