use std::{
    fs::File as fsFile,
    io::Read,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use super::{
    constants::schema::{
        cache_book, LocalCacheUserInfo, LocalCookiesInfo, UserCliConfig, YuqueLoginUserInfo,
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
                    serde_json::from_str(&data).expect("配置文件解析失败，请检查格式是否正确");
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
pub fn get_cache_books_info() -> Result<Vec<cache_book::BookItem>, bool> {
    let user_cli_config = &GLOBAL_CONFIG.books_info_file;
    if Path::new(&user_cli_config).exists() {
        match fsFile::open(user_cli_config) {
            Ok(mut f) => {
                let mut data = String::new();
                f.read_to_string(&mut data).expect("知识库文件读取失败");
                let config: cache_book::BookInfo =
                    serde_json::from_str(&data).expect("知识库文件解析失败");
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
                let config: LocalCacheUserInfo =
                    serde_json::from_str(&data).expect("用户信息缓存文件解析失败");

                Ok(config.user_info)
            }
            Err(_) => Err(false),
        }
    } else {
        Err(false)
    }
}

/// 从用户配置知识库范围获取知识库，去掉二级目录
/// # examples
/// get_top_level_toc_from_toc_range(&vec!["test-book/测试目录".to_string()])
pub fn get_top_level_toc_from_toc_range(toc_range: &Vec<String>) -> Vec<String> {
    let toc_range = &toc_range
        .iter()
        .map(|item| {
            if item.contains("/") {
                let items: Vec<&str> = item.split("/").collect();
                items[0].to_string()
            } else {
                item.to_string()
            }
        })
        .collect::<Vec<_>>();
    toc_range.clone()
}

/// 是否是要导出个人知识库
pub fn is_personal() -> bool {
    if let Ok(user_config) = get_user_config() {
        if user_config.host.is_empty() {
            return true;
        } else {
            return false;
        }
    }
    true
}
