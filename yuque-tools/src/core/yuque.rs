/*
 * Description: yuque相关的接口调用
 * Created: 2023-08-31 09:41:21
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use serde_json::{json, Value};

use std::{collections::HashMap, process};

use crate::libs::{
    constants::{schema::UserCliConfig, GLOBAL_CONFIG},
    encrypt::encrypt_password,
    file::File,
    log::Log,
    request::Request,
    tools::gen_timestamp,
};

extern crate flexbuffers;
extern crate spider;

use spider::serde::ser::Serialize;
use spider::website::Website;

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

                // for item in books_data {
                //     println!("{}", item)
                // }
                Self::get_book_docs_info(&"ss").await;

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
                    println!("{:?}", resp.to_owned());
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
        println!("----{}", repo);
        let mut website: Website = Website::new("https://rsseau.fr");

        website.crawl().await;

        let links = website.get_links();

        let mut s = flexbuffers::FlexbufferSerializer::new();

        links.serialize(&mut s).unwrap();

        println!("{:?}", s)
    }

    /// 通过下载接口获取到md文件内容
    pub async fn get_markdown_content() {
        //
    }
}

#[test]
fn test_get_book_doc_info() {
    let repo = String::from("tools");

    YuqueApi::get_book_docs_info(&repo);
}
