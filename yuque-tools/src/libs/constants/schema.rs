/*
 * Description: 用于类型结构体
 * Created: 2023-09-04 11:48:51
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LocalCookiesInfo {
    pub expire_time: u128,
    pub cookies: String,
}
