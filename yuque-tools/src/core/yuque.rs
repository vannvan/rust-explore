/*
 * Description: yuque相关的接口调用
 * Created: 2023-08-31 09:41:21
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use crate::libs::{encrypt::encrypt_password, request::Request, tools::UserConfig};
use std::collections::HashMap;
pub struct YuqueApi;

impl YuqueApi {
    pub async fn login(user_config: UserConfig) -> Result<bool, bool> {
        println!("登录语雀:{:?}", user_config);

        let url = "/api/accounts/login";
        let _password = encrypt_password(&user_config.password);
        let mut params = HashMap::new();
        params.insert("login", user_config.username);
        params.insert("password", _password);
        params.insert("loginType", "password".to_string());

        if let Ok(resp) = Request::post(&url, params).await {
            // println!("返回消息{:#?}", resp);
            if resp.get("data").is_some() {
                Ok(true)
            } else {
                Err(false)
            }
        } else {
            Err(false)
        }
    }
}
