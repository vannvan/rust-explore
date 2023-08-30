use reqwest::header::HeaderMap;
use serde_json::value::Value;
use std::collections::HashMap;

use crate::libs::{encrypt::encrypt_password, tools::UserConfig};
// use serde::Deserialize;
// use serde_json::Value;
// #![deny(warnings)]

#[allow(dead_code)]
const YUQUE_HOST: &str = "https://www.yuque.com";

const LOGIN: &str = "/api/accounts/login";

pub struct YuqueApi;

impl YuqueApi {
    pub async fn login(user_config: UserConfig) -> Result<bool, bool> {
        println!("登录语雀:{:?}", user_config);

        if let Ok(resp) = Self::post(user_config).await {
            // println!("返回消息{:#?}", resp);
            if resp.get("data").is_some() {
                Ok(true)
            } else {
                Err(false)
            }
        } else {
            Err(false)
        }
    }
    #[allow(unused)]
    pub async fn get() -> Result<HashMap<String, String>, reqwest::Error> {
        Ok(reqwest::get("https://httpbin.org/ip")
            .await?
            .json::<HashMap<String, String>>()
            .await?)
    }
    pub async fn post(user_info: UserConfig) -> Result<HashMap<String, Value>, reqwest::Error> {
        // post 请求要创建client
        let client = reqwest::Client::new();
        let referer = "https://www.yuque.com/login";
        let login_type = "password";
        // 组装header
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("referer", referer.parse().unwrap());
        headers.insert("origin", "https://www.yuque.com".parse().unwrap());

        // 组装要提交的数据
        let _password = encrypt_password(&user_info.password);
        let mut data = HashMap::new();
        data.insert("login", user_info.username);
        data.insert("password", _password);
        data.insert("loginType", login_type.to_string());

        // 发起post请求并返回
        let url = [YUQUE_HOST, LOGIN].join("");
        let res = client
            .post(&url)
            .headers(headers)
            .json(&data)
            .send()
            .await?;

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
        Ok(res.json::<HashMap<String, Value>>().await?)
    }
}
