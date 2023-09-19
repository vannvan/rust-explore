use std::{
    fs::File as fsFile,
    io::Read,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use super::{
    constants::schema::{
        BookInfo, BookItem, CacheUserInfo, LocalCookiesInfo, UserCliConfig, YuqueLoginUserInfo,
    },
    constants::GLOBAL_CONFIG,
    file::File,
};
/// 生成当前时间戳
pub fn gen_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

/// 获取本地有效cookies，如果cookies过期就返回空字符串
pub fn get_local_cookies() -> String {
    let f = File::new();
    if let Ok(cookie_info) = f.read(&GLOBAL_CONFIG.cookies_file) {
        let config: LocalCookiesInfo = serde_json::from_str(&cookie_info).expect("JSON解析失败");
        if config.expire_time < gen_timestamp() {
            String::new()
        } else {
            config.cookies
        }
    } else {
        String::new()
    }
}

/// 获取用户的CLI配置信息
pub fn get_user_config() -> Result<UserCliConfig, &'static str> {
    // println!("获取本地用户信息");
    let user_cli_config = &GLOBAL_CONFIG.user_cli_config_file;

    if Path::new(&user_cli_config).exists() {
        match fsFile::open(user_cli_config) {
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

/// 获取本地缓存的知识库信息，如果已过期就返回false
/// TODO 先去获取本地缓存的知识库，如果在半小时之内，就不用重复获取了
pub fn get_cache_books_info() -> Result<Vec<BookItem>, bool> {
    let user_cli_config = &GLOBAL_CONFIG.books_info_file;
    if Path::new(&user_cli_config).exists() {
        match fsFile::open(user_cli_config) {
            Ok(mut f) => {
                let mut data = String::new();
                f.read_to_string(&mut data).expect("知识库文件读取失败");
                let config: BookInfo = serde_json::from_str(&data).expect("知识库文件解析失败");
                Ok(config.books_info)
            }
            Err(_) => Err(false),
        }
    } else {
        Err(false)
    }
}

/// 获取缓存的用户信息
pub fn get_cache_user_info() -> Result<YuqueLoginUserInfo, bool> {
    let user_cli_config = &GLOBAL_CONFIG.user_info_file;
    if Path::new(&user_cli_config).exists() {
        match fsFile::open(user_cli_config) {
            Ok(mut f) => {
                let mut data = String::new();
                f.read_to_string(&mut data)
                    .expect("用户信息缓存文件读取失败");
                let config: CacheUserInfo =
                    serde_json::from_str(&data).expect("用户信息缓存文件解析失败");

                Ok(config.user_info)
            }
            Err(_) => Err(false),
        }
    } else {
        Err(false)
    }
}
