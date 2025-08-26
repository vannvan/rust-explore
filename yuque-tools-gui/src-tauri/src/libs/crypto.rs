use base64::{engine::general_purpose, Engine as _};
use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Encrypt, RsaPublicKey};

/// 加密工具模块
pub struct CryptoUtils;

impl CryptoUtils {
    /// 加密密码（使用语雀的RSA加密方式）
    pub fn encrypt_password(password: &str) -> String {
        const RSA_2048_PUB_PEM: &str = include_str!("yuque.pem");
        let pub_key = RsaPublicKey::from_public_key_pem(RSA_2048_PUB_PEM).unwrap();
        let mut rng = rand::thread_rng();

        // 构造密码格式：时间戳:密码
        let password_with_timestamp = [
            super::http_utils::HttpUtils::gen_timestamp().to_string(),
            ":".to_string(),
            password.to_string(),
        ]
        .join("");

        // 使用RSA公钥加密
        let enc_data = pub_key
            .encrypt(
                &mut rng,
                Pkcs1v15Encrypt,
                password_with_timestamp.as_bytes(),
            )
            .unwrap();

        // 返回base64编码的加密结果
        general_purpose::STANDARD.encode(enc_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_encryption() {
        let password = "test_password";
        let encrypted = CryptoUtils::encrypt_password(password);

        assert!(!encrypted.is_empty());
        assert_ne!(encrypted, password);
        assert!(encrypted.len() > password.len());

        println!("原始密码: {}", password);
        println!("加密结果: {}", encrypted);
    }
}
