use std::fmt::{Display, Formatter};

use derive_builder::Builder;
use reqwest::Client;
use serde::{ Deserialize, Serialize};

/*

miniprogrampage	object	否	小程序卡片，msgtype="miniprogrampage" 时必填
*/
use crate::RPayResult;

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub struct CustomerService {
    /// 接口调用凭证，该参数为 URL 参数，非 Body 参数。使用getAccessToken 或者 authorizer_access_token
    #[builder(setter(into))]
    pub access_token: String,
    // 用户的 OpenID
    #[builder(setter(into))]
    pub touser: String,
    /// 消息类型。text表示文本消息；image表示图片消息；link表示图文链接；miniprogrampage表示小程序卡片。
    #[builder(setter(into))]
    #[serde(rename = "msgtype")]
    pub msg_type: MsgType,
    /// 文本消息，msgtype="text" 时必填
    #[builder(default, setter(into))]
    pub text: Option<Text>,
    ///  图片消息，msgtype="image" 时必填
    #[builder(default, setter(into))]
    pub image : Option<Image>,
    /// 图文链接，msgtype="link" 时必填
    #[builder(default, setter(into))]
    pub link: Option<Link>,
    /// 小程序卡片，msgtype="miniprogrampage" 时必填
    #[builder(default, setter(into))]
    pub miniprogrampage: Option<Miniprogrampage>,
}

impl CustomerService {

    pub async fn send(&mut self) -> RPayResult<Response> {
        let url = format!("https://api.weixin.qq.com/cgi-bin/message/custom/send?access_token={}", self.access_token);
        let json_body = serde_json::to_string(self).unwrap();
        println!("{}", json_body);
        let resp = Client::new()
            .post(url)
            .body(json_body)
            .send()
            .await?
            .json::<Response>()
            .await?;
        Ok(resp)
    }

    pub async fn get_thumb_media_id(&mut self, _text: String) -> RPayResult<Response> {
        let url = format!("https://api.weixin.qq.com/cgi-bin/material/add_material?access_token={}&type=thumb", self.access_token);
        let resp = Client::new()
            .post(url)
            .send()
            .await?
            .json::<Response>()
            .await?;
        println!("{:?}", resp);
        Ok(resp)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub errcode: i64,
    pub errmsg: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Miniprogrampage {
    /// 消息标题
    pub title: Option<String>,
    /// 小程序的页面路径，跟app.json对齐，支持参数，比如pages/index/index?foo=bar
    pub pagepath: Option<String>,
    /// 小程序消息卡片的封面， image 类型的 media_id，通过 uploadTempMedia接口上传图片文件获得，建议大小为 520*416
    pub thumb_media_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    /// 消息标题
    pub title: Option<String>,
    /// 图文链接消息
    pub description: Option<String>,
    /// 图文链接消息被点击后跳转的链接
    pub url: Option<String>,
    /// 图文链接消息的图片链接，支持 JPG、PNG 格式，较好的效果为大图 640 X 320，小图 80 X 80
    pub thumb_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    // 发送的图片的媒体ID，通过 uploadTempMedia上传图片文件获得。
    pub media_id: String,
}

// 文本消息，msgtype="text" 时必填
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    // content 文本消息内容。msgtype="text" 时必填
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum  MsgType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "miniprogrampage")]
    Miniprogrampage
}

impl Display for MsgType {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MsgType::Text => write!(fmt, "text"),
            MsgType::Link => write!(fmt, "link"),
            MsgType::Miniprogrampage => write!(fmt, "miniprogrampage"),
        }
    }
}