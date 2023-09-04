/*
 * Description: 调度
 * Created: 2023-08-31 09:41:21
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use serde::Deserialize;
use std::{fs::File, io::Read, path::Path, process};

use crate::{
    core::yuque::YuqueApi,
    libs::{log::Log, tools::get_local_cookies},
};

#[derive(Debug, Deserialize)]
pub struct UserConfig {
    pub username: String,
    pub password: String,
}

pub struct Scheduler {
    //
}
impl Scheduler {
    pub async fn start() -> Result<(), &'static str> {
        let cookies = get_local_cookies();

        if cookies.is_empty() {
            match Self::get_user_config() {
                Ok(user_config) => {
                    if let Ok(_resp) = Self::login_yuque_and_save_cookies(user_config).await {
                        Log::success("登录成功!")
                    } else {
                        Log::error("登录失败");
                        process::exit(1)
                    }
                }
                Err(err) => Log::error(err),
            }
        } else {
            // 有cookie，不走登录
            // println!("cookies-> {}", cookies);
            YuqueApi::get_user_bookstacks().await
        }
        Ok(())
    }

    /// 获取用户配置信息
    pub fn get_user_config() -> Result<UserConfig, &'static str> {
        // println!("获取本地用户信息");
        let exit_file = "yuque.config.json";

        if Path::new(&exit_file).exists() {
            match File::open(exit_file) {
                Ok(mut f) => {
                    let mut data = String::new();
                    f.read_to_string(&mut data).expect("配置文件读取失败");
                    let config: UserConfig = serde_json::from_str(&data).expect("JSON解析失败");
                    Ok(config)
                }
                Err(_) => Err("配置文件读取失败"),
            }
        } else {
            Err("配置文件不存在")
        }
    }
    /// 登录语雀并存储cookies
    pub async fn login_yuque_and_save_cookies(user_config: UserConfig) -> Result<(), bool> {
        if let Ok(_e) = YuqueApi::login(user_config).await {
            Ok(())
        } else {
            Err(false)
        }
    }
}
