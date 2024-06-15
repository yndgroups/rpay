use aes_gcm::{aead::{AeadMut, Payload}, Aes256Gcm, KeyInit};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::{ model::WechatPayDecodeData, utils, RPayError, RPayResult};
use super::config::WechatV3PayConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub struct ParseEncrypt {
    #[builder(default="String::new()",setter(into))]
    pub ciphertext: String,
    #[builder(default="String::new()",setter(into))]
    pub nonce: String,
    #[builder(default="String::new()",setter(into))]
    pub associated_data: String,
    #[builder(default="String::new()",setter(into))]
    pub algorithm: String,
    #[builder(default="String::new()",setter(into))]
    pub original_type: String,
}

/// 创建请求
impl ParseEncrypt {
    
    /// 解密支付回调参数
    pub async fn parse(&mut self, wechat_sdk: WechatV3PayConfig) -> RPayResult<WechatPayDecodeData> {
        if self.nonce.len() != 12 {
            return Err(RPayError::ErrorWithMsg(String::from("nonce长度必须为12")));
        }
        let api_key_v3 = &wechat_sdk.api_key_v3.unwrap_or_default();
        let ciphertext = utils::base64_decode(self.ciphertext.clone())?;
        let aes_key = api_key_v3.as_bytes();
        let mut cipher = Aes256Gcm::new(aes_key.into());
        let payload = Payload {
            msg: &ciphertext.as_slice(),
            aad: &self.associated_data.as_bytes(),
        };
        let plain_text = cipher
            .decrypt(self.nonce.as_bytes().into(), payload)
            .map_err(|e| RPayError::ErrorWithMsg(e.to_string()))?;
        Ok(serde_json::from_slice(&plain_text)?)
    }
}