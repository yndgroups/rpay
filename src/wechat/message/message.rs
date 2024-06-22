use std::collections::HashMap;

use derive_builder::Builder;
use reqwest::Client;
use serde::{ Deserialize, Serialize};
use serde_json::Value;

use crate::{core::common::Lang, RPayResult};

/*
 {
  "touser": "OPENID",
  "template_id": "TEMPLATEID",
  "page": "mp.weixin.qq.com",
  "miniprogram":{
        "":"APPID",
        // 公众号
        "pagepath":"index?foo=bar"
        // 小程序
        "pagepath": { "value": any }
  },
  "data": {
      "name1": {
          "value": "广州腾讯科技有限公司"
      },
      "thing8": {
          "value": "广州腾讯科技有限公司"
      },
       "time7": {
          "value": "2019年8月8日"
      }
     }
}
 */
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub struct Message {
    /// 接口调用凭证
    #[builder(setter(into))]
    pub access_token: String,
    // 接收者（用户）的 openid
    #[builder(setter(into))]
    pub touser: String,
    /// 所需下发的订阅模板id
    #[builder(setter(into))]
    pub template_id: String,
    /// 点击模板卡片后的跳转页面，仅限本小程序内的页面。支持带参数,（示例index?foo=bar）。该字段不填则模板无跳转
    #[builder(default,setter(into))]
    pub page: Option<String>,
    #[builder(default,setter(into))]
    pub mini_program: Option<MiniProgram>,
    /// 是	模板内容，格式形如 { "key1": { "value": any }, "key2": { "value": any } }
    #[builder(setter(into))]
    pub data: HashMap<String, DataItem>,
    /// 跳转小程序类型：developer为开发版；trial为体验版；formal为正式版；默认为正式版
    #[builder(default,setter(into))]
    pub miniprogram_state: String,
    #[builder(default,setter(into))]
    pub lang: Lang,
}

// 进入小程序查看”的语言类型，支持zh_CN(简体中文)、en_US(英文)、zh_HK(繁体中文)、zh_TW(繁体中文)，默认为zh_CN

#[derive(Debug, Clone, Serialize, Deserialize,Default)]
pub enum MiniProgramState {
    #[serde(rename = "developer")]
    #[default]
    Developer,
    #[serde(rename = "trial")]
    Trial,
    #[serde(rename = "formal")]
    Formal
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgram {
    #[serde(rename = "appid")]
    pub app_id: String,
    #[serde(rename = "pagepath")]
    pub page_path: Value
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataItem {
    pub value: String,
}

impl Message {

    // send发送订阅通知
    // https://developers.weixin.qq.com/doc/offiaccount/Subscription_Messages/api.html#send发送订阅通知
    pub async fn send(&mut self) -> RPayResult<Response> {
        let url = format!("https://api.weixin.qq.com/cgi-bin/message/subscribe/send?access_token={}", self.access_token);
        let json_body = serde_json::to_string(self).unwrap();
        let resp = Client::new().post(url)
            .body(json_body)
            .send()
            .await?
            .json::<Response>()
            .await?;
        Ok(resp)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct Response {
    pub errmsg: String,
    pub errcode: i64
}