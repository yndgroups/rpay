use derive_builder::Builder;
use serde::{ Deserialize, Serialize};

use crate::{constant::CGI_BIN_TOKEN_WECHAT_HOST, core::request::RequestBuilder, RPayResult};


/// accessToken 登录授权
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub struct AccessToken {
    ///第三方用户唯一凭证
    #[serde(rename = "appid")]
    #[builder(setter(into))]
    pub app_id: String,
    /// 第三方用户唯一凭证密钥，即app secret
    #[serde(rename = "secret")]
    #[builder(setter(into))]
    pub secret: String,
    // 获取access_token填写client_credential
}

impl AccessToken {
    /// 异步请求微信服务器获取访问令牌。
    ///
    /// 该方法构建一个请求，用于向微信服务器发送申请访问令牌的请求。它根据appid和secret计算请求URL，
    /// 然后发送请求并解析响应。
    ///
    /// # 参数
    /// 无参数
    ///
    /// # 返回值
    /// 返回一个`RPayResult<Response>`，其中`Response`是请求的结果，包含访问令牌等信息。
    /// 如果请求失败，会返回错误信息。
    pub async fn request(&mut self) -> RPayResult<Response> {
        // 根据appid和secret构造获取访问令牌的URL
        let url = format!("{}?grant_type=client_credential&appid={}&secret={}", CGI_BIN_TOKEN_WECHAT_HOST, self.app_id, self.secret);
        // 构建请求，发送到微信服务器，并等待响应
        let resp = RequestBuilder::default().url(url).build()?.send::<Response>().await?;
         // 返回响应结果
        Ok(resp)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct Response {
    /// 获取到的凭证
    pub access_token: Option<String>,
    /// 凭证有效时间，单位：秒
    pub expires_in: Option<i64>,
    /// 错误code 返回码	说明
    /// -1	系统繁忙，此时请开发者稍候再试
    /// 0	请求成功
    /// 40001	AppSecret错误或者AppSecret不属于这个公众号，请开发者确认AppSecret的正确性
    /// 40002	请确保grant_type字段值为client_credential
    /// 40164	调用接口的IP地址不在白名单中，请在接口IP白名单中进行设置。
    /// 40243	AppSecret已被冻结，请登录MP解冻后再次调用。
    /// 89503	此IP调用需要管理员确认,请联系管理员
    /// 89501	此IP正在等待管理员确认,请联系管理员
    /// 89506	24小时内该IP被管理员拒绝调用两次，24小时内不可再使用该IP调用
    /// 89507	1小时内该IP被管理员拒绝调用一次，1小时内不可再使用该IP调用
    pub errcode: Option<i64>,
    /// 错误信息
    errmsg: Option<String>
}