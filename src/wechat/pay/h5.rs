use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{constant::V3_PAY_TRANSACTIONS_H5, core::common::RPayResponse, model::{Amount, Detail, SettleInfo, StoreInfo}, utils, RPayResult};

use super::config::WechatV3PayConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub struct H5Pay {
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
    // pub payer: Payer,
    /// 选填 Detail 【优惠功能】 优惠功能
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Detail>,
    /// 必填 SceneInfo【场景信息】支付场景描述
    #[builder(setter(into))]
    pub scene_info: H5ReqSceneInfo,
    /// 选填 SettleInfo【结算信息】 结算信息
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<SettleInfo>,
}

impl H5Pay {
    pub async fn pay(&mut self, wechat_sdk: WechatV3PayConfig) -> RPayResult<H5Response> {
        self.app_id = wechat_sdk.app_id.clone();
        self.mch_id = wechat_sdk.mch_id.clone();
        self.notify_url = wechat_sdk.notify_url.clone().unwrap_or_default();
        let json_body = serde_json::to_string(self).unwrap();
        utils::build_request::<H5Response>(wechat_sdk,crate::common::HttpMethod::POST, V3_PAY_TRANSACTIONS_H5, json_body).await
    }
}

///【场景信息】
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct H5ReqSceneInfo {
    ///【用户终端IP】必填 用户的客户端IP，支持IPv4和IPv6两种格式的IP地址。
    pub payer_client_ip: String,
    ///【商户端设备号】 商户端设备号（门店号或收银设备ID）。
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    ///【商户门店信息】 商户门店信息
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_info: Option<StoreInfo>,
    ///【H5场景信息】
    pub h5_info: H5Info,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct H5Info {
    ///【场景类型】 场景类型 必填(32)
    #[serde(rename = "type")]
    pub h5_type: String,
    ///【应用名称】 应用名称
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    ///【网站URL】 网站URL
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_url: Option<String>,
    ///【iOS平台BundleID】 iOS平台BundleID
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    ///【Android平台PackageName】 Android平台PackageName
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct H5Response {
    pub code: Option<String>,
    pub message: Option<String>,
    ///【二维码链接】 此URL用于生成支付二维码，然后提供给用户扫码支付。
    /// 注意：code_url并非固定值，使用时按照URL格式转成二维码即可。
    pub h5_url: Option<String>,
}

impl RPayResponse for H5Response {}
