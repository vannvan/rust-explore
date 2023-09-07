/*
 * Description: yuque相关的接口调用
 * Created: 2023-08-31 09:41:21
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use crate::core::scheduler::UserConfig;
use crate::libs::constants::GLOBAL_CONFIG;
use crate::libs::file::File;
use crate::libs::log::Log;
use crate::libs::{encrypt::encrypt_password, request::Request};
use std::collections::HashMap;
pub struct YuqueApi;

#[allow(dead_code)]
impl YuqueApi {
    /// 登录语雀
    pub async fn login(user_config: UserConfig) -> Result<bool, bool> {
        // println!("登录语雀:{:?}", user_config);
        let _password = encrypt_password(&user_config.password);
        let mut params = HashMap::new();
        params.insert("login", user_config.username);
        params.insert("password", _password);
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
    pub async fn get_user_bookstacks() {
        Log::info(&"开始获取知识库");
        if let Ok(resp) = Request::get(&GLOBAL_CONFIG.yuque_book_stacks).await {
            if resp.get("data").is_some() {
                let mut books_data = vec![];
                let flat = resp.get("data").unwrap();

                for item in flat.as_array().unwrap() {
                    for sub_item in item.to_owned().get("books").unwrap().as_array().unwrap() {
                        // println!("{:?}", sub_item)
                        books_data.push(sub_item.to_owned())
                    }
                }

                let f = File::new();

                let _ = f.write(
                    ".meta/booksinfo.json",
                    serde_json::to_string(&books_data).into_iter().collect(),
                );
                // println!("{:?}", serde_json::to_string(&books).unwrap())
            } else {
                println!("获取失败")
            }
        }
    }

    /// 获取知识库下文档数据
    pub async fn get_book_docs_info() {
        //
    }

    /// 通过下载接口获取到md文件内容
    pub async fn get_markdown_content() {
        //
    }
}
