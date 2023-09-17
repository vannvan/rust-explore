/*
 * Description: yuque相关的接口调用
 * Created: 2023-08-31 09:41:21
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use regex::Regex;
use rsa::pkcs8::der::asn1::Null;
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
                let f = File::new();

                let user_info = json!( {
                    "expire_time": gen_timestamp() + GLOBAL_CONFIG.local_expire,
                    "user_info": resp.get("data").unwrap().get("user").unwrap()
                });

                match f.write(&GLOBAL_CONFIG.user_info_file, user_info.to_string()) {
                    Ok(_) => (),
                    Err(err) => {
                        if cfg!(debug_assertions) {
                            println!("写入用户信息失败信息：{}", err)
                        }
                        Log::error("缓存目录创建失败");
                        process::exit(1)
                    }
                }
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
                // TODO 待调整
                let current_login = "vannvan";

                for item in data_wrap.as_array().unwrap() {
                    for sub_item in item.to_owned().get("books").unwrap().as_array().unwrap() {
                        let current_book_user_login =
                            sub_item.get("user").unwrap().get("login").unwrap();
                        // 知识库所属
                        let book_type = if current_login == current_book_user_login.to_string() {
                            "owner"
                        } else {
                            "collab"
                        };

                        let book_info = json!({
                          "name": sub_item.get("name"),
                          "slug": sub_item.get("slug"),
                          "stack_id": sub_item.get("stack_id"),
                          "book_id": sub_item.get("id"),
                          "user_login": sub_item.get("user").unwrap().get("login"),
                          "user_name": sub_item.get("user").unwrap().get("name"),
                          "book_type": book_type
                        });

                        books_data.push(book_info)
                    }
                }

                let f = File::new();

                for item in &mut books_data {
                    let user_login = item["user_login"].as_str().unwrap_or_default();
                    let slug = item["slug"].as_str().unwrap_or_default();
                    let url = format!("/{}/{}", user_login, slug);
                    let ss = Self::get_book_docs_info(&url).await;

                    if let Ok(book_toc) = ss {
                        item["docs"] = book_toc;
                    }
                }

                let books_info = json!({
                    "expire_time": gen_timestamp() + GLOBAL_CONFIG.local_expire,
                    "books_info": books_data
                });

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
    pub async fn get_book_docs_info(repo: &str) -> Result<Value, Null> {
        if let Ok(resp) = Self::crawl_book_toc_info(repo).await {
            let sss = resp.get("book").unwrap().get("toc").unwrap();
            Ok(sss.clone())
        } else {
            Err(Null)
        }
    }

    /// 爬取知识库
    async fn crawl_book_toc_info(url: &str) -> Result<Value, reqwest::Error> {
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

        // let reg = Regex::new(r#"decodeURIComponent.*""#).unwrap();
        // Regex::new(r#"decodeURIComponent\("([^"]+)"\)"#).unwrap()
        // if let Some(captures) = reg.captures(&res_text.to_string()) {
        //     let re = Regex::new(r#"".*""#).unwrap();
        //     let caps = re.captures(captures.get(0).unwrap().as_str());
        // let decoded: String = parse(
        //     caps.unwrap()
        //         .get(0)
        //         .unwrap()
        //         .to_owned()
        //         .as_str()
        //         .replace(r#"""#, "")
        //         .as_bytes(),
        // )
        //     .map(|(key, val)| [key, val].concat())
        //     .collect();

        //     let parsed: Value = serde_json::from_str(&decoded).unwrap();

        //     // println!("{:?}", parsed["book"]["toc"])
        //     // let sssss = parsed["book"]["toc"];
        //     return Ok(parsed);
        // } else {
        //     println!("No match found");
        //     return Ok(Value::String("".to_owned()));
        // }
        // 优化
        let reg = Regex::new(r#"decodeURIComponent\("([^"]+)"\)"#).unwrap();
        if let Some(captured) = reg.captures(&res_text.to_string()) {
            let decoded = &captured[1];

            let decoded1: String = parse(decoded.as_bytes())
                .map(|(key, val)| [key, val].concat())
                .collect();
            let parsed: Value = serde_json::from_str(&decoded1).unwrap();
            // println!("Decoded value: {}", decoded);
            Ok(parsed)
        } else {
            println!("No match found");
            return Ok(Value::String("{}".to_owned()));
        }
    }

    /// 通过下载接口获取到md文件内容
    pub async fn get_markdown_content(url: &str) -> Result<String, reqwest::Error> {
        println!("{}", url);
        Ok("()".to_string())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_crawl_success() {
        let ss = YuqueApi::crawl_book_toc_info("/vannvan/dd67e4").await;
        match ss {
            Ok(resp) => {
                if !resp["book"].is_null() {
                    // println!("返回结果:{:?}", resp["book"]["toc"])
                    let toc_list = &resp["book"]["toc"];
                    for item in toc_list.as_array().unwrap() {
                        println!("-- {:?}", item)
                    }
                }
            }
            Err(_) => todo!(),
        }
    }
    #[tokio::test]
    async fn test_crawl_fail() {
        let ss = YuqueApi::crawl_book_toc_info("/vannvan/dd67").await;

        match ss {
            Ok(resp) => {
                if resp["book"].is_null() {
                    println!("返回数据为空")
                }
            }
            Err(err) => {
                println!("{:?}", err)
            }
        }
    }
    #[test]
    fn reg() {
        let input = r#"window.appData = JSON.parse(decodeURIComponent("%7B%22me%22%3A%7B%22PERMISSION%22%3A"))"#;

        let re = Regex::new(r#"decodeURIComponent\("([^"]+)"\)"#).unwrap();
        if let Some(captured) = re.captures(input) {
            let decoded = &captured[1];
            println!("Decoded value: {}", decoded);
        } else {
            println!("No match found");
        }
    }
}
