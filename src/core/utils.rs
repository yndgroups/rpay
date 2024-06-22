use base64::engine::general_purpose;
use base64::{DecodeError, Engine};
use derive_builder::Builder;
use reqwest::header::{HeaderMap, CONTENT_TYPE, USER_AGENT};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use crate::common::HttpMethod;
use crate::constant::{ACCEPT, AUTHORIZATION};
use crate::pay::config::WechatV3PayConfig;
use crate::{utils, RPayError, RPayResult};
use rsa::pkcs8::DecodePrivateKey;
use rsa::sha2::Digest;
use rsa::{Pkcs1v15Sign, RsaPrivateKey};

// 获取字符串
pub fn get_nonce_str() -> String {
    Uuid::new_v4().to_simple().to_string()
}

// 获取时间戳
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

/// 构建请求
pub async fn build_pay_request<T: DeserializeOwned>(wechat_sdk: WechatV3PayConfig, method: HttpMethod, url: &str, body: String) -> RPayResult<T> {
        let headers = build_header(&wechat_sdk,method.clone(), url, body.clone())?;
        let client = Client::new();
        let url = format!("https://api.mch.weixin.qq.com{}", url);
        let builder = match method {
            HttpMethod::GET => client.get(url),
            HttpMethod::POST => client.post(url),
            HttpMethod::PUT => client.put(url),
            HttpMethod::DELETE => client.delete(url),
            HttpMethod::PATCH => client.patch(url),
        };
        builder
            .headers(headers)
            .body(body)
            .send()
            .await?
            .json::<T>()
            .await
            .map(Ok)?
}

// 构建请求头信息
pub fn build_header(
    sdk: &WechatV3PayConfig,
    method: HttpMethod,
    url: impl AsRef<str>,
    body: impl AsRef<str>,
) -> RPayResult<HeaderMap> {
    let method = method.to_string();
    let url = url.as_ref();
    let body = body.as_ref();
    let timestamp = chrono::Local::now().timestamp();
    let serial_no = sdk.serial_no.clone().unwrap_or("".to_string());
    let mch_id = sdk.mch_id.clone().to_string();
    let nonce_str = Uuid::new_v4().to_string().replace("-", "").to_uppercase();
    let signature = format!(
        "{}\n{}\n{}\n{}\n{}\n",
        method, url, timestamp, nonce_str, body,
    );
    let signature = utils::sha256_sign(sdk.private_key.clone().unwrap_or_default(), signature)?;
    let authorization = format!("WECHATPAY2-SHA256-RSA2048 mchid=\"{mch_id}\",nonce_str=\"{nonce_str}\",signature=\"{signature}\",timestamp=\"{timestamp}\",serial_no=\"{serial_no}\"");
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    let chrome_agent = "Mozilla/5.0 (Linux; Android 10; Redmi K30 Pro) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.198 Mobile Safari/537.36";
    headers.insert(USER_AGENT, chrome_agent.parse().unwrap());
    match chrome_agent.parse() {
        Ok(d) => {
            headers.insert(USER_AGENT, d);
        }
        Err(err) => return Err(RPayError::ErrorWithMsg(err.to_string())),
    }
    match authorization.parse() {
        Ok(d) => {
            headers.insert(AUTHORIZATION, d);
        }
        Err(err) => return Err(RPayError::ErrorWithMsg(err.to_string())),
    }
    match "application/json".parse() {
        Ok(d) => {
            headers.insert(CONTENT_TYPE, d);
        }
        Err(err) => return Err(RPayError::ErrorWithMsg(err.to_string())),
    }
    Ok(headers)
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub struct Request {
    #[builder(setter(into))]
    pub url: String,
    #[builder(default,setter(into))]
    #[serde(skip)]
    pub headers: Option<HeaderMap>,
    #[builder(default,setter(into))]
    pub body: Option<String>,
    #[builder(default,setter(into))]
    #[serde(skip)]
    method: HttpMethod
}

impl Request  {
    /// 构建请求
pub async fn send<T: DeserializeOwned>(&self) -> RPayResult<T> {
    let client = Client::new();
    let mut builder = match self.method {
        HttpMethod::GET => client.get(&self.url),
        HttpMethod::POST => client.post(&self.url),
        HttpMethod::PUT => client.put(&self.url),
        HttpMethod::DELETE => client.delete(&self.url),
        HttpMethod::PATCH => client.patch(&self.url),
    };
    if let Some(headers) = &self.headers {
        builder = builder.headers(headers.clone())
    }
    if let Some(body) = &self.body {
        builder = builder.body(body.to_string())
    }
    builder
        .send()
        .await?
        .json::<T>()
        .await
        .map(Ok)?
}

}