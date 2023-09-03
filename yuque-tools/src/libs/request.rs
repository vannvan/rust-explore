/*
 * Description:
 * Created: 2023-08-31 18:47:09
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

// use config::Config;
use regex::Regex;
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::collections::HashMap;

use crate::libs::constants::GLOBAL_CONFIG;

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

    #[allow(unused)]
    pub async fn get(url: &str) -> Result<HashMap<String, String>, reqwest::Error> {
        Ok(reqwest::get(url)
            .await?
            .json::<HashMap<String, String>>()
            .await?)
    }

    pub async fn post(
        url: &str,
        params: HashMap<&str, String>,
    ) -> Result<HashMap<String, Value>, reqwest::Error> {
        let client = reqwest::Client::new();
        let header = Self::request_header();

        let target_url = [&GLOBAL_CONFIG.yuque_host, url].join("");
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

            println!("cookies->  {}", cookies);
        }

        Ok(res.json::<HashMap<String, Value>>().await?)
    }
}
