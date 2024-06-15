use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::{model::{AppResult, JsapiResult, PayType}, utils, RPayError, RPayResult};

use super::config::WechatV3PayConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub struct PayInfo {
    // 下单的id
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepay_id: Option<String>,
    /// 支付跳转链接（H5支付 会返回）
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h5_url: Option<String>,
    /// 二维码链接（NATIVE支付 会返回）
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_url: Option<String>,
    /// 支付类型
    #[builder(setter)]
    pub pay_type: PayType,
}

impl PayInfo {
    /// 获取支付签名
    pub async fn signature(&self, wechat_sdk: WechatV3PayConfig) -> RPayResult<Value> {
        let timestamp = chrono::Local::now().timestamp();
        let nonce_str = Uuid::new_v4().to_string().replace("-", "").to_uppercase();
        match self.pay_type {
            PayType::H5 => Ok(Value::String(
                self.h5_url.to_owned().unwrap_or_default(),
            )),
            PayType::Jsapi => {
                let prepay_id = self.prepay_id.clone().unwrap_or_default();
                let mut result = JsapiResult {
                    app_id: wechat_sdk.app_id.clone(),
                    time_stamp: timestamp.to_string(),
                    nonce_str,
                    prepay_id: prepay_id.clone(),
                    package: format!("prepay_id={}", prepay_id),
                    sign_type: "RSA".to_string(),
                    pay_sign: String::default(),
                };
                result.pay_sign = utils::sha256_sign(
                    wechat_sdk.private_key.clone().unwrap_or_default(),
                    result.clone().get_sign_str(),
                )?;
                Ok(serde_json::to_value(result)?)
            }
            PayType::Native => Ok(Value::String(
                self.code_url.to_owned().unwrap_or_default(),
            )),
            PayType::App => {
                let mut result = AppResult {
                    partner_id: wechat_sdk.mch_id.clone(),
                    appid: wechat_sdk.app_id.clone(),
                    time_stamp: timestamp.to_string(),
                    nonce_str,
                    package_value: format!("Sign=WXPay"),
                    prepay_id: self.prepay_id.clone().unwrap_or_default(),
                    sign: "".to_string(),
                };
                result.sign = utils::sha256_sign(
                    wechat_sdk.private_key.clone().unwrap_or_default(),
                    result.clone().get_sign_str(),
                )?;
                Ok(serde_json::to_value(result)?)
            }
            PayType::Micro => Err(RPayError::ErrorWithMsg(String::from("该类型暂不支持"))),
        }
    }
}
