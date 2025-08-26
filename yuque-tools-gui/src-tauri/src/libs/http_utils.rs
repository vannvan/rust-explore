/*
 * Description:
 * Created: 2025-08-26 11:34:21
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */
use super::constants::Headers;
use reqwest::Client;
use std::time::{SystemTime, UNIX_EPOCH};

/// HTTP工具模块
pub struct HttpUtils;

impl HttpUtils {
    /// 构建请求头
    pub fn build_headers(cookies: &[String]) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();

        // 使用常量构建请求头
        headers.insert("Content-Type", Headers::CONTENT_TYPE.parse().unwrap());
        headers.insert("referer", Headers::REFERER.parse().unwrap());
        headers.insert("origin", Headers::ORIGIN.parse().unwrap());
        headers.insert("User-Agent", Headers::USER_AGENT_MOBILE.parse().unwrap());

        // 添加 cookies 如果存在
        if !cookies.is_empty() {
            let cookie_header = &cookies[0];
            println!("Debug: 使用的cookie头: {}", cookie_header);
            headers.insert("Cookie", cookie_header.parse().unwrap());
        }

        headers
    }

    /// 创建HTTP客户端
    pub fn create_client() -> Client {
        Client::builder()
            .user_agent(Headers::USER_AGENT_DESKTOP)
            .build()
            .unwrap_or_default()
    }

    /// 生成当前时间戳
    pub fn gen_timestamp() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_generation() {
        let timestamp1 = HttpUtils::gen_timestamp();
        let timestamp2 = HttpUtils::gen_timestamp();

        assert!(timestamp2 >= timestamp1);

        println!("时间戳1: {}", timestamp1);
        println!("时间戳2: {}", timestamp2);
    }
}
