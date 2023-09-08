use std::time::{SystemTime, UNIX_EPOCH};

use super::{constants::schema::LocalCookiesInfo, constants::GLOBAL_CONFIG, file::File};
/// 生成时间戳
pub fn gen_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

/// 获取本地有效cookies
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
