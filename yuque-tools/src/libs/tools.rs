use std::time::{SystemTime, UNIX_EPOCH};

/// 生成时间戳
pub fn gen_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}
