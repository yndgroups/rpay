use base64::engine::general_purpose;
use base64::{DecodeError, Engine};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use crate::{utils, RPayError, RPayResult};
use rsa::pkcs8::DecodePrivateKey;
use rsa::sha2::Digest;
use rsa::{Pkcs1v15Sign, RsaPrivateKey};

// 获取字符串
#[allow(unused)]
pub fn get_nonce_str() -> String {
    Uuid::new_v4().to_simple().to_string()
}

// 获取时间戳
#[allow(unused)]
pub fn get_timestamp() -> RPayResult<i64> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) =>  {
            let time = d .as_secs() as i64 * 1000i64 + (d.subsec_nanos() as f64 / 1_000_000.0) as i64;
            Ok(time)
        },
        Err(err) => Err(RPayError::ErrorWithMsg(format!("时间获取失败{}", err.to_string())))
    }
}

// 随机订单号
#[allow(unused)]
pub fn random_trade_no() -> String {
    Uuid::new_v4().to_simple().to_string()
}

/// base64解码
#[allow(unused)]
pub fn base64_encode<S>(content: S) -> String
where
    S: AsRef<[u8]>,
{
    general_purpose::STANDARD.encode(content)
}

/// base64编码
#[allow(unused)]
pub fn base64_decode<S>(content: S) -> Result<Vec<u8>, DecodeError>
where
    S: AsRef<[u8]>,
{
    general_purpose::STANDARD.decode(content.as_ref())
}

/// 加密信息
#[allow(unused)]
pub fn sha256_sign(private_key: String, content: String) -> Result<String, RPayError> {
    println!("sha256_sign => private_key, {}, content: {}", private_key, content);
    match RsaPrivateKey::from_pkcs8_pem(&private_key) {
        Ok(psk) => {
            let hasher = rsa::sha2::Sha256::new()
                .chain_update(content)
                .finalize();
            let padding = Pkcs1v15Sign::new::<rsa::sha2::Sha256>();
            match  psk.sign(padding, &hasher) {
                Ok(sn) => Ok(utils::base64_encode(sn)),
                Err(_) => Err(RPayError::ErrorWithMsg(String::from("签名失败"))),
            }            
        }
        Err(err) => Err(RPayError::ErrorWithMsg(format!("私钥解析失败:{:?}", err))),
    }
}

 /// 创建签名信息
 #[allow(unused)]
 pub fn rsa_sign(content: String, private_key: String) -> RPayResult<String> {
    Ok(utils::sha256_sign(private_key, content)?)
}