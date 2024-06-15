
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
/// 微信支付client
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub struct WechatV3PayConfig {
    /// 微信id
    #[builder(setter(into))]
    pub app_id: String,
    /// 秘钥
    #[builder(setter(into))]
    pub secret: String,
    /// 商户编号
    #[builder(setter(into))]
    pub mch_id: String,
    /// 私钥 V3
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_v3: Option<String>,
    /// 私钥
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// 通知地址
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    /// 退货通知地址
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_notify_url: Option<String>,
    /// API证书序列号
    #[builder(setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<String>,
    /// API商户证书秘钥
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    // 证书文件
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkcs12_path: Option<String>,
}
