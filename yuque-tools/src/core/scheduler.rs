/*
 * Description: 调度
 * Created: 2023-08-31 09:41:21
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use std::{fs::File, io::Read, path::Path, process};

use crate::{
    core::yuque::YuqueApi,
    libs::{
        constants::{schema::UserCliConfig, GLOBAL_CONFIG},
        log::Log,
        tools::get_local_cookies,
    },
};

pub struct Scheduler;
impl Scheduler {
    pub async fn start() -> Result<(), &'static str> {
        let cookies = get_local_cookies();

        if cookies.is_empty() {
            match Self::get_user_config() {
                Ok(user_config) => {
                    match YuqueApi::login(user_config).await {
                        Ok(_resp) => {
                            Log::success("登录成功!");
                            // 接着就开始获取知识库
                            if let Ok(_books_info) = YuqueApi::get_user_bookstacks().await {
                                Log::success("获取知识库成功")
                            }
                        }
                        Err(_err) => {
                            Log::error("登录失败，请检查账号信息是否正确或重试");
                            process::exit(1)
                        }
                    }
                }
                Err(err) => Log::error(err),
            }
        } else {
            // 有cookie，不走登录
            // println!("cookies-> {}", cookies);
            if let Ok(_books_info) = YuqueApi::get_user_bookstacks().await {
                Log::success("获取知识库成功");
                // print!("{:?}", serde_json::from_value(_books_info).unwrap())
            }
        }
        Ok(())
    }

    /// 获取用户配置信息
    pub fn get_user_config() -> Result<UserCliConfig, &'static str> {
        // println!("获取本地用户信息");
        let exit_file = &GLOBAL_CONFIG.user_cli_config_file;

        if Path::new(&exit_file).exists() {
            match File::open(exit_file) {
                Ok(mut f) => {
                    let mut data = String::new();
                    f.read_to_string(&mut data).expect("配置文件读取失败");
                    let config: UserCliConfig =
                        serde_json::from_str(&data).expect("配置文件解析失败，请检查字段是否完整");
                    Ok(config)
                }
                Err(_) => Err("配置文件读取失败"),
            }
        } else {
            Err("配置文件不存在")
        }
    }
}
