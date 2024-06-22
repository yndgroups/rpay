use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        common::{GrantType, LoginType},
        request::RequestBuilder,
    },
    RPayResult,
};

/// 小程序授权
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub struct Auth {
    ///第三方用户唯一凭证
    #[serde(rename = "appid")]
    #[builder(setter(into))]
    pub app_id: String,
    /// 第三方用户唯一凭证密钥，即app secret
    #[serde(rename = "secret")]
    #[builder(setter(into))]
    pub secret: String,
    // 微信code
    #[serde(rename = "wx_code")]
    #[builder(setter(into))]
    pub wx_code: String,
    // 微信code
    #[serde(rename = "wx_code")]
    #[builder(setter(into))]
    pub grant_type: GrantType,
    // 登录类型
    #[serde(rename = "wx_code")]
    #[builder(setter(into))]
    pub login_type: LoginType,
}

impl Auth {
    /// 公众号登录 https://developers.weixin.qq.com/doc/offiaccount/OA_Web_Apps/Wechat_webpage_authorization.html
   pub async fn web_login(&self) -> RPayResult<WebLoginResponse>  {
        let url = format!("https://api.weixin.qq.com/sns/oauth2/access_token?appid={}&secret={}&code={}&grant_type=authorization_code", self.app_id,self.secret, self.wx_code);
        // 构建请求，发送到微信服务器，并等待响应
        let resp = RequestBuilder::default()
            .url(url)
            .build()?
            .send::<WebLoginResponse>()
            .await?;
        // 返回响应结果
        Ok(resp)
    }

    /// 刷新access_token
    /// https://developers.weixin.qq.com/doc/offiaccount/OA_Web_Apps/Wechat_webpage_authorization.html
    pub async fn web_refresh_token(&self,refresh_token: &str) -> RPayResult<WebRefreshTokenResponse> {
        let url = format!("https://api.weixin.qq.com/sns/oauth2/refresh_token?appid={}&grant_type=refresh_token&refresh_token={}", self.app_id,refresh_token);
        // 构建请求，发送到微信服务器，并等待响应
        let resp = RequestBuilder::default()
            .url(url)
            .build()?
            .send::<WebRefreshTokenResponse>()
            .await?;
        // 返回响应结果
        Ok(resp)
    }

    /// 公众号登录之后获取的登录用户信息
    /// https://developers.weixin.qq.com/doc/offiaccount/OA_Web_Apps/Wechat_webpage_authorization.html
    pub async fn web_sn_sapi_user_info(&self,access_token: String, openid: &str) -> RPayResult<WeLoginUserResponse> {
        let url = format!("https://api.weixin.qq.com/sns/userinfo?access_token={}&openid={}&lang=zh_CN", access_token, openid);
        // 构建请求，发送到微信服务器，并等待响应
        let resp = RequestBuilder::default()
            .url(url)
            .build()?
            .send::<WeLoginUserResponse>()
            .await?;
        // 返回响应结果
        Ok(resp)
    }

    /// 小程序登录  https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/code2Session.html
    pub async fn mini_program_login(&self) -> RPayResult<MiniProgramResponse> {
        let url = format!("https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code", self.app_id,self.secret, self.wx_code);
        // 构建请求，发送到微信服务器，并等待响应
        let resp = RequestBuilder::default()
            .url(url)
            .build()?
            .send::<MiniProgramResponse>()
            .await?;
        // 返回响应结果
        Ok(resp)
    }
}

/// 公众号登录之后获取的登录用户信息
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct WeLoginUserResponse {
    /// 用户标识
    pub openid: Option<String>,
    /// 用户昵称
    pub nickname: Option<String>,
    /// 用户性别
    pub sex: Option<i64>,
    /// 用户所在国家
    pub province: Option<String>,
    /// 用户所在城市
    pub city: Option<String>,
    /// 用户所在国家
    pub country: Option<String>,
    /// 用户头像，最后一个数值代表正方形头像大小（有0、46、64、96、132数值可选，0代表640*640正方形头像），用户没有头像时该项为空。若用户更换头像，原有头像URL将失效。
    pub headimgurl: Option<String>,
    /// 用户特权信息，json 数组，如微信沃卡用户为（chinaunicom）
    pub privilege: Option<Vec<String>>,
    /// 用户统一标识。针对一个微信开放平台帐号下的应用，同一用户的 unionid 是唯一的。
    pub unionid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct WebRefreshTokenResponse {
    /// 获取到的凭证
    pub access_token: String,
    /// 凭证有效时间，单位：秒
    pub expires_in: i64,
    /// 刷新令牌
    pub refresh_token: String,
    /// 用户标识
    pub openid: String,
    /// 用户授权作用域
    pub scope: String,
     /// 错误码 错误时微信会返回JSON数据包如下（示例为Code无效错误）
     pub errcode: Option<i32>,
     /// 错误信息 错误时微信会返回JSON数据包如下（示例为Code无效错误）
     pub errmsg: Option<String>,
}

/// 微信公众号登录
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct WebLoginResponse {
    /// 网页授权接口调用凭证,注意：此access_token与基础支持的access_token不同
    pub access_token: Option<String>,
    /// access_token接口调用凭证超时时间，单位（秒）
    pub expires_in: Option<i64>,
    /// 用户刷新access_token
    pub refresh_token: Option<String>,
    /// 用户唯一标识，请注意，在未关注公众号时，用户访问公众号的网页，也会产生一个用户和公众号唯一的OpenID
    pub openid: Option<String>,
    /// 用户授权的作用域，使用逗号（,）分隔
    pub scope: Option<String>,
    /// 是否为快照页模式虚拟账号，只有当用户是快照页模式虚拟账号时返回，值为1
    pub is_snapshotuser: Option<i64>,
    /// 用户统一标识（针对一个微信开放平台账号下的应用，同一用户的 unionid 是唯一的），只有当scope为"snsapi_userinfo"时返回
    pub unionid: Option<String>,
    /// 错误码 错误时微信会返回JSON数据包如下（示例为Code无效错误）
    pub errcode: Option<i32>,
    /// 错误信息 错误时微信会返回JSON数据包如下（示例为Code无效错误）
    pub errmsg: Option<String>,
}

// 小程序登录返回信息
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct MiniProgramResponse {
    // session_key	string	会话密钥
    pub session_key: Option<String>,
    // unionid	string	用户在开放平台的唯一标识符，若当前小程序已绑定到微信开放平台账号下会返回，详见 UnionID 机制说明。
    pub unionid: Option<String>,
    // errmsg	string	错误信息
    pub errmsg: Option<String>,
    // openid	string	用户唯一标识
    pub openid: Option<String>,
    // errcode	int32	错误码
    pub errcode: Option<i32>,
}