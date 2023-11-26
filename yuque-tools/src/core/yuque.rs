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
use spinoff::{spinners, Color, Spinner};

use std::{collections::HashMap, process};

use crate::libs::{
    constants::GLOBAL_CONFIG,
    encrypt::encrypt_password,
    file::File,
    log::Log,
    request::Request,
    tools::{gen_timestamp, get_cache_user_info, is_personal},
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
    pub async fn login(username: &str, password: &str) -> Result<bool, bool> {
        // println!("登录语雀:{:?}", user_config);
        let _password = encrypt_password(&password);
        let mut params = HashMap::new();
        params.insert("login", username.to_string());
        params.insert("password", _password.to_string());
        params.insert("loginType", "password".to_string());

        if let Ok(resp) = Request::post(&GLOBAL_CONFIG.mobile_login, params).await {
            if resp.get("data").is_some() {
                let f = File::new();
                let user_info = json!({
                    "expire_time": gen_timestamp() + GLOBAL_CONFIG.local_expire,
                    "user_info": resp.get("data").unwrap().get("me").unwrap()
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

    /// 获取个人知识库/团队知识库列表数据
    pub async fn get_user_bookstacks() -> Result<Value, bool> {
        let is_personal = is_personal();
        Log::info("开始获取知识库");
        // loading开始
        let mut spinner =
            Spinner::new(spinners::Dots, "正在获取知识库数据，请稍后...", Color::Blue);

        let target_api = if is_personal {
            &GLOBAL_CONFIG.yuque_book_stacks
        } else {
            &GLOBAL_CONFIG.yuque_space_books_info
        };

        if cfg!(debug_assertions) {
            println!("获取知识库地址：{}", target_api);
        }

        if let Ok(resp) = Request::get(&target_api).await {
            if resp.get("data").is_some() {
                let data_wrap = resp.get("data").unwrap();
                let f = File::new();

                let filtered_books_data = if is_personal {
                    let docs = Self::gen_books_data_for_cache(&data_wrap).await;
                    docs
                } else {
                    let mut temp_books_data: Vec<Value> = vec![];
                    // 构造一个 [{books:[...]}] 结构的数据
                    let books_info = json!({ "books": data_wrap });
                    temp_books_data.push(books_info);

                    let docs =
                        Self::gen_books_data_for_cache(&serde_json::Value::Array(temp_books_data))
                            .await;

                    docs
                };

                let mut merged_books_data = vec![];

                if let Ok(collab_books) = Self::get_collab_books().await {
                    let collab_books_array = collab_books.to_owned();
                    for book in collab_books_array.as_array().unwrap() {
                        merged_books_data.push(book.clone());
                    }
                };

                for book in filtered_books_data.as_array().unwrap() {
                    merged_books_data.push(book.clone());
                }

                let books_info = json!({
                    "expire_time": gen_timestamp() + GLOBAL_CONFIG.local_expire,
                    "books_info": merged_books_data
                });

                // 写入知识库信息文件
                match f.write(&GLOBAL_CONFIG.books_info_file, books_info.to_string()) {
                    Err(_) => {
                        Log::error("文件创建失败");
                        process::exit(1)
                    }
                    Ok(_) => {
                        // loading结束
                        spinner.stop();
                        Ok(books_info)
                    }
                }
                // println!("{:?}", serde_json::to_string(&books).unwrap())
            } else {
                if cfg!(debug_assertions) {
                    println!("获取知识库响应信息：{:?}", resp.to_owned());
                }
                let mut error_info = String::from("获取知识库失败: ");
                error_info.push_str(resp.get("message").unwrap().to_string().as_str());
                spinner.stop();
                Log::error(&error_info);
                Err(false)
            }
        } else {
            spinner.stop();
            Log::error("获取知识库失败");
            Err(false)
        }
    }

    /// 获取协作知识库数据
    pub async fn get_collab_books() -> Result<Value, bool> {
        if let Ok(resp) = Request::get(&GLOBAL_CONFIG.yuque_collab_books_info).await {
            if resp.get("data").is_some() {
                let data_wrap = resp.get("data").unwrap();
                let mut temp_books_data: Vec<Value> = vec![];
                // 构造一个 [{books:[...]}] 结构的数据
                let books_info = json!({ "books": data_wrap });
                temp_books_data.push(books_info);

                let docs =
                    Self::gen_books_data_for_cache(&serde_json::Value::Array(temp_books_data))
                        .await;

                // println!("协作知识库：{:?}", docs);
                Ok(docs)
            } else {
                Err(false)
            }
        } else {
            Err(false)
        }
    }

    /// 生成适配缓存结构的知识库数据
    pub async fn gen_books_data_for_cache(book_info: &Value) -> Value {
        let mut target_books_data = vec![];

        let current_login = get_cache_user_info().unwrap().login.to_string();

        for item in book_info.as_array().unwrap() {
            for sub_item in item.to_owned().get("books").unwrap().as_array().unwrap() {
                let current_book_user_login = sub_item.get("user").unwrap().get("login").unwrap();
                if cfg!(debug_assertions) {
                    println!(
                        "当前登录用户 {}, 当前知识库用户 {},{}",
                        current_login.to_string(),
                        current_book_user_login,
                        current_login.to_string() == current_book_user_login.to_owned()
                    );
                }

                // 知识库所属
                let book_type = if current_login == current_book_user_login.to_owned() {
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

                target_books_data.push(book_info)
            }
        }

        for item in &mut target_books_data {
            let user_login = item["user_login"].as_str().unwrap_or_default();
            let slug = item["slug"].as_str().unwrap_or_default();
            let url = format!("/{}/{}", user_login, slug);
            let toc = Self::get_book_docs_info(&url).await;

            if let Ok(book_toc) = toc {
                item["docs"] = book_toc;
            }
        }

        // println!("{:?}", target_books_data);
        // 过滤掉可能没有文档的知识库
        let filtered_books_data: Vec<serde_json::Value> = target_books_data
            .into_iter()
            .filter(|item| !item["docs"].is_null())
            .collect();

        serde_json::Value::Array(filtered_books_data)
    }

    /// 爬取知识库下文档数据
    pub async fn get_book_docs_info(repo: &str) -> Result<Value, Null> {
        if let Ok(resp) = Self::crawl_book_toc_info(repo).await {
            if let Some(book) = resp.get("book") {
                if let Some(toc) = book.get("toc") {
                    return Ok(toc.clone());
                }
            }
        }
        Err(Null)
    }

    /// 爬取知识库
    async fn crawl_book_toc_info(url: &str) -> Result<Value, reqwest::Error> {
        match Request::get_text(&url).await {
            Ok(res_text) => {
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
            Err(err) => Err(err),
        }
    }

    /// 通过下载接口获取到md文件内容
    pub async fn get_markdown_content(url: &str, line_break: bool) -> Result<String, Null> {
        let line_break = line_break;

        let query = format!(
            "attachment=true&latexcode=false&anchor=false&linebreak={}",
            line_break
        );

        let target_url = format!("{}/markdown?{}", url, query);
        match Request::get_text(&target_url).await {
            Ok(content) => {
                if content.is_empty() {
                    Err(Null)
                } else {
                    Ok(content)
                }
            }
            Err(_) => Err(Null),
        }
    }

    // 获取团队资源基本信息
    pub async fn get_group_resource_base_info() -> Result<Value, bool> {
        if let Ok(resp) = Request::get(&GLOBAL_CONFIG.group_resource_base_info).await {
            if resp.get("data").is_some() {
                // println!("{:?}", resp.get("data").unwrap())
                Ok(resp.get("data").unwrap().to_owned())
            } else {
                Err(false)
            }
        } else {
            Err(false)
        }
    }

    /// 获取资源详情列表
    pub async fn get_group_resource_detail_list(id: &str) -> Result<Value, bool> {
        let url = format!(
            "/api/groups/{}/books?q=&archived=include&type=Design%2CResource",
            id
        );

        if let Ok(resp) = Request::get(&url).await {
            if resp.get("data").is_some() {
                Ok(resp.get("data").unwrap().to_owned())
            } else {
                // println!("获取资源详情列表【{}】失败{:?}", id, resp);
                Err(false)
            }
        } else {
            Err(false)
        }
    }

    /// 获取资源列表，需要层层往下找
    pub async fn get_group_resource_list(id: &str, parent_id: Option<&str>) -> Result<Value, bool> {
        // let url = format!("/api/resources?book_id={}&offset=0", id);

        let url = match parent_id {
            Some(parent_id) => {
                format!(
                    "/api/resources?book_id={}&parent_id={}&offset=0",
                    id, parent_id
                )
            }
            None => {
                format!("/api/resources?book_id={}&offset=0", id)
            }
        };

        if let Ok(resp) = Request::get(&url).await {
            if resp.get("data").is_some() {
                // Ok(resp.get("data").unwrap().to_owned())
                let list = resp.get("data");
                for resource_item in list.unwrap().as_array().unwrap() {
                    println!(
                        "资源列表：{:?},type - {:?},book_id - {:?}, id - {:?}",
                        resource_item.get("filename"),
                        resource_item.get("type"),
                        resource_item.get("book_id"),
                        resource_item.get("id")
                    );
                    if resource_item
                        .get("type")
                        .unwrap()
                        .as_str()
                        .eq(&Some("folder"))
                    {
                        if let Ok(sub) = Self::get_group_resource_list(
                            &resource_item.get("book_id").unwrap().to_string(),
                            Some(&resource_item.get("id").unwrap().to_string()),
                        )
                        .await
                        {
                            return Ok(sub.get("data").unwrap().to_owned());
                            // return Ok(sub.get("data").unwrap().to_owned());
                        }
                    }
                }
                Ok(resp.get("data").unwrap().to_owned())
            } else {
                // println!("获取资源列表【{}】失败{:?}", id, resp);
                Err(false)
            }
        } else {
            Err(false)
        }
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
        let doc_info = YuqueApi::crawl_book_toc_info("/vannvan/dd67").await;

        match doc_info {
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
    fn reg_toc_info() {
        let input = r#"window.appData = JSON.parse(decodeURIComponent("%7B%22me%22%3A%7B%22PERMISSION%22%3A"))"#;

        let re = Regex::new(r#"decodeURIComponent\("([^"]+)"\)"#).unwrap();
        if let Some(captured) = re.captures(input) {
            let decoded = &captured[1];
            println!("Decoded value: {}", decoded);
        } else {
            println!("No match found");
        }
    }

    #[tokio::test]
    async fn test_get_doc_content() {
        if let Ok(content) =
            YuqueApi::get_markdown_content(&"/vannvan/dd67e4/fogcsik8cxgvnodw".to_string(), true)
                .await
        {
            println!("{}", content);
            assert_eq!(content, "二级子文档\n")
        } else {
            panic!("内容获取失败")
        }
    }
}
