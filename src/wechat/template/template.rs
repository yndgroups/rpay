use derive_builder::Builder;
use serde::{ Deserialize, Serialize};
use serde_json::json;

use crate::{core::request::RequestBuilder, RPayResult};


#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub struct Templates {
    #[builder(setter(into))]
    pub access_token: String
}

impl Templates {
    // https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-message-management/subscribe-message/deleteMessageTemplate.html
    pub async fn delete(&mut self, template_id: String) -> RPayResult<Response> {
        let body = json!({
            "priTmplId": template_id
        });
        println!("body = {}", body);
        let url = format!("https://api.weixin.qq.com/wxaapi/newtmpl/deltemplate?access_token={}", self.access_token);
        let resp = RequestBuilder::default()
            .url(url)
            .body(body.to_string())
            .build()?.send::<Response>().await?;
        Ok(resp)
    }

    // 获取类目
    pub async fn get_category(&self) -> RPayResult<TemplatesResponse<Vec<CategoryResponse>>> {
        let url = format!("https://api.weixin.qq.com/wxaapi/newtmpl/getcategory?access_token={}", self.access_token);
        let resp = RequestBuilder::default()
            .url(url)
            .build()?.send().await?;
        Ok(resp)
    }

    /// 获取关键词列表
    /// 属性	类型	必填	说明
    // access_token	string	是	接口调用凭证，该参数为 URL 参数，非 Body 参数。使用access_token或者authorizer_access_token
    // tid	string	是	模板标题 id，可通过接口获取
    pub async fn get_pub_template_keywords(&self, tid: String) -> RPayResult<TemplatesResponse<Vec<KeywordsResponse>>> {
        let url = format!("https://api.weixin.qq.com/wxaapi/newtmpl/getpubtemplatekeywords?access_token={}&tid={}", self.access_token, tid);
        let resp = RequestBuilder::default()
            .url(url)
            .build()?.send().await?;
        Ok(resp)
    }
}

// 获取类目返回数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  KeywordsResponse {
    // id，选用模板时需要
    pub kid: i64,
    // 关键词内容
    pub name: String,
    // 关键词内容对应的示例
    pub example: String,
    // 参数类型
    pub rule: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct Response {
    pub errmsg: String,
    pub errcode: i64
}

// 获取类目返回数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  CategoryResponse {
    // 类目id，查询公共库模版时需要
    pub id: i64,
    // 类目的中文名
    pub name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatesResponse<T> {
    /// 错误码
    pub errmsg: String,
    // 错误信息
    pub errcode: i64,
    // 返回数据
    pub data: Option<T>
}