/*
 * Description: 加密
 * Created: 2023-08-30 11:33:49
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */
use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Encrypt, RsaPublicKey};

use std::time::{SystemTime, UNIX_EPOCH};

#[allow(deprecated)]
pub fn encrypt_password(target_str: &str) -> String {
    const RSA_2048_PUB_PEM: &str = include_str!("yuque.pem");
    let pub_key = RsaPublicKey::from_public_key_pem(RSA_2048_PUB_PEM).unwrap();
    let mut rng = rand::thread_rng();

    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();

    // println!("{}", &time);
    let password = [time, ":".to_string(), target_str.to_string()].join("");

    let enc_data = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, password.as_bytes())
        .unwrap();

    // &base64::encode(enc_data).to_string()
    base64::encode(enc_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let s = encrypt_password(&"hello".to_string());
        println!("密码 {:?}", s)
    }
}
