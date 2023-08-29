use super::tools::UserConfig;
use serde::Deserialize;
use serde_json::Value;
// #![deny(warnings)]

#[allow(dead_code)]
const YUQUE_HOST: &str = "https://www.yuque.com";

const LOGIN: &str = "/api/accounts/login";

pub struct YuqueApi;

impl YuqueApi {
    pub async fn login(user_config: UserConfig) -> bool {
        println!("登录语雀:{:?}", user_config);

        // let url = String::from(login);

        // let res = reqwest::post(url).await?.json::<Response<Data>>().await?;

        // let body = res.text().await?;

        let params = [("foo", "bar"), ("baz", "quux")];
        let client = reqwest::Client::new();
        let res = client
            .post("http://httpbin.org/post")
            .form(&params)
            .send()
            .await;

        println!("sssss{:?}", res);

        if res.is_ok() {
            true
        } else {
            false
        }
    }
}
