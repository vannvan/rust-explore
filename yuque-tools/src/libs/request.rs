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
use reqwest::{header::HeaderMap, Response};

use crate::libs::{
    constants::GLOBAL_CONFIG,
    file::File,
    log::Log,
    tools::{gen_timestamp, get_local_cookies, get_user_config},
};
use serde_json::{json, Value};
use std::{fs::File as fsFile, io::Write};

use std::{collections::HashMap, process};

#[allow(dead_code)]
pub fn crawl() {
    //
}

pub struct Request {
    pub host: String,
}

impl Request {
    // 获取匹配的host，如果是个人就用配置，如果是用户指定的就用指定的
    fn get_match_host() -> String {
        if let Ok(user_config) = get_user_config() {
            // user_config.host
            if user_config.host.is_empty() {
                GLOBAL_CONFIG.yuque_host.clone()
            } else {
                user_config.host
            }
        } else {
            GLOBAL_CONFIG.yuque_host.clone()
        }
    }

    fn request_header() -> HeaderMap {
        // 组装header
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("referer", GLOBAL_CONFIG.yuque_referer.parse().unwrap());
        headers.insert("origin", Self::get_match_host().parse().unwrap());
        headers.insert("User-Agent", GLOBAL_CONFIG.user_agent.parse().unwrap());
        return headers;
    }
    /// 返回JSON
    pub async fn get(url: &str) -> Result<HashMap<String, Value>, reqwest::Error> {
        let target_url = Self::get_match_host().clone() + &url;
        if cfg!(debug_assertions) {
            println!("GET-> {}", &target_url);
        }
        // let res = reqwest::get(&target_url);
        // Ok(res.json::<HashMap<String, String>>().await?)
        let client = reqwest::Client::new();

        let cookies = get_local_cookies();

        if cookies.is_empty() {
            Log::error("cookies已过期，请清除缓存后重新执行程序");
            process::exit(1)
        }

        let res = client
            .get(target_url)
            .header("cookie", cookies)
            .header("content-type", "application/json")
            .header("x-requested-with", "XMLHttpRequest")
            .send()
            .await?;

        let res_status = res.status().as_u16();

        let resp = res.json::<HashMap<String, Value>>().await?;
        if res_status != 200 {
            Log::error(format!("接口请求失败：{}", url).as_str());
            Log::error(format!("{:?}", resp).as_str());
        }
        Ok(resp)
    }

    /// 返回响应文本
    pub async fn get_text(url: &str) -> Result<String, reqwest::Error> {
        let target_url = Self::get_match_host().clone() + &url;
        if cfg!(debug_assertions) {
            println!("GET-> {}", &target_url);
        }
        // let res = reqwest::get(&target_url);
        // Ok(res.json::<HashMap<String, String>>().await?)
        let client = reqwest::Client::new();

        let cookies = get_local_cookies();

        if cookies.is_empty() {
            Log::error("cookies已过期，请清除缓存后重新执行程序");
            process::exit(1)
        }

        let res = client
            .get(target_url)
            .header("cookie", cookies)
            .header("content-type", "application/json")
            .header("x-requested-with", "XMLHttpRequest")
            .send()
            .await?;

        Ok(res.text().await?)
    }

    pub async fn post(
        url: &str,
        params: HashMap<&str, String>,
    ) -> Result<HashMap<String, Value>, reqwest::Error> {
        let client = reqwest::Client::new();
        let header = Self::request_header();
        let target_url = Self::get_match_host().clone() + &url;
        let login_reg = Regex::new("login");
        if cfg!(debug_assertions) {
            println!("POST-> {}", &target_url);
        }

        let res = client
            .post(target_url)
            .headers(header)
            .json(&params)
            .send()
            .await?;

        let res_status = res.status().as_u16();

        // 如果是登录，就存下cookies
        if login_reg.unwrap().is_match(url) && res_status == 200 {
            Self::save_cookies(&res)
        }

        let resp = res.json::<HashMap<String, Value>>().await?;

        if res_status != 200 {
            Log::error(format!("接口请求失败：{}", url).as_str());
            Log::error(format!("{:?}", resp).as_str());
        }

        Ok(resp)
    }

    /// 下载文件
    pub async fn download(url: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let cookies = get_local_cookies();

        let target_url = Self::get_match_host().clone() + &url;
        // if cfg!(debug_assertions) {
        //     println!("下载链接: {}, 文件名称: {}", target_url, filename)
        // }
        let mut response = client
            .get(target_url)
            .header("cookie", cookies)
            .send()
            .await?;

        // if cfg!(debug_assertions) {
        //     println!("下载响应：{:?}", response);
        // }

        let mut dest = fsFile::create(filename)?;
        while let Some(chunk) = response.chunk().await? {
            dest.write_all(&chunk)?;
        }
        Ok(())
    }

    fn save_cookies(res: &Response) {
        let cookies: String = res
            .headers()
            .iter()
            .filter_map(|(key, value)| {
                if key.to_string().to_lowercase() == "set-cookie" {
                    value.to_str().ok()
                } else {
                    None
                }
            })
            .collect::<Vec<&str>>()
            .join(";");

        let cookies_info = json!({
            "expire_time": gen_timestamp() + GLOBAL_CONFIG.local_expire,
            "cookies": cookies,
        });

        let f = File::new();

        if let Err(_) = f.mkdir(&GLOBAL_CONFIG.meta_dir) {
            Log::error("缓存目录创建失败");
            process::exit(1)
        }

        if let Err(_) = f.write(&GLOBAL_CONFIG.cookies_file, cookies_info.to_string()) {
            Log::error("缓存暂存失败");
            process::exit(1);
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download() {
        if let Ok(_res) = Request::download(
            "https://cdn.nlark.com/yuque/0/2023/png/1553840/1699433154733-638a24c2-3ad0-4427-aae4-851dc4cbebc2.png",
            "./dev/下载文件.png",
        )
        .await
        {
            println!("下载成功")
        } else {
            println!("下载失败")
        }
    }
}
