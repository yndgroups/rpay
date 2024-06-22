use crate::RPayError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub trait RPayResponse: DeserializeOwned {}
pub type RPayResult<T> = std::result::Result<T, RPayError>;

// 进入小程序查看”的语言类型，支持zh_CN(简体中文)、en_US(英文)、zh_HK(繁体中文)、zh_TW(繁体中文)，默认为zh_CN
#[derive(Clone, Debug,Copy, Serialize, Deserialize, Default)]
pub enum Lang {
    #[default]
    #[serde(rename = "zh_CN")]
    ZhCn,
    #[serde(rename = "en_US")]
    EnUs,
    #[serde(rename = "zh_HK")]
    ZhHk,
    #[serde(rename = "zh_TW")]
    ZhTw,
}

// 授权类型
#[derive(Clone, Debug,Copy, Serialize, Deserialize)]
pub enum GrantType {
    #[serde(rename  = "authorization_code")]
    AuthorizationCode,
}

// 微信登录类型
#[derive(Clone, Debug,Copy, Serialize, Deserialize)]
pub enum LoginType {
    Web,
    MiniProgram,
    App,
}