/*
 * Description: yuque相关的接口调用
 * Created: 2023-08-31 09:41:21
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use regex::Regex;
use serde_json::{json, Value};

use std::{collections::HashMap, process};

use crate::libs::{
    constants::{schema::UserCliConfig, GLOBAL_CONFIG},
    encrypt::encrypt_password,
    file::File,
    log::Log,
    request::Request,
    tools::{gen_timestamp, get_local_cookies},
};
use url::form_urlencoded::parse;

extern crate flexbuffers;

#[derive(PartialEq, Eq, Hash)]
struct YuqueUser<'a> {
    pub login: &'a str,
    pub name: &'a str,
}
// #[derive(PartialEq, Eq, Hash)]
#[allow(dead_code)]
struct BookInfo<'a> {
    pub name: &'a str,
    pub slug: &'a str,
    pub stack_id: &'a str,
    pub user: YuqueUser<'a>,
}

pub struct YuqueApi;

#[allow(dead_code)]
impl YuqueApi {
    /// 登录语雀并存储cookies
    pub async fn login(user_config: UserCliConfig) -> Result<bool, bool> {
        // println!("登录语雀:{:?}", user_config);
        let _password = encrypt_password(&user_config.password);
        let mut params = HashMap::new();
        params.insert("login", user_config.username.to_string());
        params.insert("password", _password.to_string());
        params.insert("loginType", "password".to_string());

        if let Ok(resp) = Request::post(&GLOBAL_CONFIG.yuque_login, params).await {
            if resp.get("data").is_some() {
                Ok(true)
            } else {
                Err(false)
            }
        } else {
            Err(false)
        }
    }

    /// 获取知识库列表数据
    pub async fn get_user_bookstacks() -> Result<Value, bool> {
        Log::info("开始获取知识库");
        if let Ok(resp) = Request::get(&GLOBAL_CONFIG.yuque_book_stacks).await {
            if resp.get("data").is_some() {
                let mut books_data = vec![];
                let data_wrap = resp.get("data").unwrap();

                for item in data_wrap.as_array().unwrap() {
                    for sub_item in item.to_owned().get("books").unwrap().as_array().unwrap() {
                        let book_info = json!({
                          "name": sub_item.get("name"),
                          "slug": sub_item.get("slug"),
                          "stack_id": sub_item.get("stack_id"),
                          "book_id": sub_item.get("id"),
                          "user_login": sub_item.get("user").unwrap().get("login"),
                          "user": {
                            "name": sub_item.get("user").unwrap().get("name"),
                            "login": sub_item.get("user").unwrap().get("login")
                          }
                        });

                        books_data.push(book_info)
                    }
                }

                let f = File::new();

                let books_info = json!({
                    "expire_time": gen_timestamp() + GLOBAL_CONFIG.local_expire,
                    "booksInfo": books_data
                });

                for item in books_data {
                    // println!("{}", &item.get("slug") + &item.get("user_login"))
                    // Self::get_book_docs_info(&"/vannvan/dd67e4").await;
                }

                // 写入知识库信息文件
                match f.write(&GLOBAL_CONFIG.books_info_file, books_info.to_string()) {
                    Err(_) => {
                        Log::error("文件创建失败");
                        process::exit(1)
                    }
                    Ok(_) => Ok(books_info),
                }
                // println!("{:?}", serde_json::to_string(&books).unwrap())
            } else {
                if cfg!(debug_assertions) {
                    println!("获取知识库响应信息：{:?}", resp.to_owned());
                }
                let mut error_info = String::from("获取知识库失败: ");
                error_info.push_str(resp.get("message").unwrap().to_string().as_str());
                Log::error(&error_info);
                Err(false)
            }
        } else {
            Log::error("获取知识库失败");
            Err(false)
        }
    }

    /// 获取知识库下文档数据
    pub async fn get_book_docs_info(repo: &str) {
        match Self::crawl_book_toc_info(repo).await {
            Ok(resp) => {
                println!("响应内容：{}", resp)
            }
            Err(_err) => {
                //
            }
        };
    }

    async fn crawl_book_toc_info(url: &str) -> Result<String, reqwest::Error> {
        let target_url = GLOBAL_CONFIG.yuque_host.clone() + &url;
        if cfg!(debug_assertions) {
            println!("GET-> {}", &target_url);
        }
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

        let res_text = res.text().await?;

        let reg = Regex::new(r#"decodeURIComponent.*""#).unwrap();
        if let Some(captures) = reg.captures(&res_text.to_string()) {
            let re = Regex::new(r#"".*""#).unwrap();
            let caps = re.captures(captures.get(0).unwrap().as_str());
            let decoded: String = parse(
                caps.unwrap()
                    .get(0)
                    .unwrap()
                    .to_owned()
                    .as_str()
                    .replace(r#"""#, "")
                    .as_bytes(),
            )
            .map(|(key, val)| [key, val].concat())
            .collect();

            let parsed: Value = serde_json::from_str(&decoded).unwrap();

            println!("{:?}", parsed["book"]["toc"])
        } else {
            println!("No match found");
        }

        Ok((&"").to_string())
    }

    /// 通过下载接口获取到md文件内容
    pub async fn get_markdown_content(url: &str) -> Result<String, reqwest::Error> {
        println!("{}", url);
        Ok("()".to_string())
    }
}
