use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};

use super::yuque::YuqueApi;

#[derive(Debug, Deserialize)]
pub struct UserConfig {
    pub username: String,
    pub password: String,
}

pub struct Tools {
    //
}
impl Tools {
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

    pub fn login_yuque_and_save_cookies(
        user_config: UserConfig,
    ) -> Result<&'static str, &'static str> {
        let s = YuqueApi::login(user_config);
        println!("{}", String::from(&s));
        if Some(s).is_some() {
            Ok("ss")
        } else {
            Err("dd")
        }
    }
}
