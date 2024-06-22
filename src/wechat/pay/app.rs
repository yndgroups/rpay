use derive_builder::Builder;
use serde::{ Deserialize, Serialize};

use crate::{core::{common::RPayResponse, request::Request}, model::{Amount, Detail, Payer, SceneInfo, SettleInfo, SignData}, RPayResult};

use super::config::WechatV3PayConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub struct AppPay {
    /// 【公众号ID】 公众号ID (必填:不能长度大于32个字)
    #[serde(rename = "appid")]
    #[builder(setter(into))]
    pub app_id: String,
    /// 【直连商户号】 直连商户号(必填:不能长度大于32个字)
    #[serde(rename = "mchid")]
    #[builder(setter(into))]
    pub mch_id: String,
    /// 商品描述 (必填:不能长度大于127个字)
    #[builder(setter(into))]
    pub description: String,
    /// 商户系统内部订单号，(必填:不能长度大于32个字)只能是数字、大小写字母_-*且在同一个商户号下唯一。
    #[builder(setter(into))]
    pub out_trade_no: String,
    ///【交易结束时间】(选填:不能长度大于64个字) 订单失效时间，遵循rfc3339标准格式，格式为yyyy-MM-DDTHH:mm:ss+TIMEZONE，yyyy-MM-DD表示年月日，T出现在字符串中，表示time元素的开头，HH:mm:ss表示时分秒，TIMEZONE表示时区（+08:00表示东八区时间，领先UTC8小时，即北京时间）。例如：2015-05-20T13:29:35+08:00表示，北京时间2015年5月20日13点29分35秒。
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    /// 【附加数据】(选填:不能长度大于128个字) 附加数据，在查询API和支付通知中原样返回，可作为自定义参数使用，实际情况下只有支付完成状态才会返回该字段。
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    /// 【通知地址】 (选填:不能长度大于255个字) 异步接收微信支付结果通知的回调地址，通知URL必须为外网可访问的URL，不能携带参数。 公网域名必须为HTTPS，如果是走专线接入，使用专线NAT IP或者私有回调域名可使用HTTP
    #[builder(setter(into))]
    pub notify_url: String,
    /// 【订单优惠标记】 选填(32) 订单优惠标记
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    /// 【电子发票入口开放标识】选填 boolean 传入true时，支付成功消息和支付详情页将出现开票入口。需要在微信支付商户平台或微信公众平台开通电子发票功能，传此字段才可生效。
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_fapiao: Option<bool>,
    ///必填 Amount 【订单金额】 订单金额信息
    pub amount: Amount,
    /// 必填 Payer【支付者】 支付者信息
    pub payer: Payer,
    /// 选填 Detail 【优惠功能】 优惠功能
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Detail>,
    /// 选填 SceneInfo【场景信息】支付场景描述
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<SceneInfo>,
    /// 选填 SettleInfo【结算信息】 结算信息
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<SettleInfo>,
}

/// 创建请求
impl AppPay {
     pub async fn pay(&mut self, wechat_sdk: WechatV3PayConfig) -> RPayResult<AppResponse> {
        self.app_id = wechat_sdk.app_id.clone();
        self.mch_id = wechat_sdk.mch_id.clone();
        self.notify_url = wechat_sdk.notify_url.clone().unwrap_or_default();
        let json_body = serde_json::to_string(self).unwrap();
        Request::build_pay_request::<AppResponse>(wechat_sdk,crate::common::HttpMethod::POST, "/v3/pay/transactions/app", json_body).await
    }
}


#[derive(Debug, Deserialize)]
pub struct AppResponse {
    /// 状态码
    pub code: Option<String>,
    /// 回答信息
    pub message: Option<String>,
    ///【预支付交易会话标识】 预支付交易会话标识。用于后续接口调用中使用，该值有效期为2小时
    pub prepay_id: Option<String>,
    ///【签名数据】
    pub sign_data: Option<SignData>,
}

impl RPayResponse for AppResponse {}
