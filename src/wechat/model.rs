use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// 【订单金额】 订单金额信息
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]

pub struct Amount {
    /// 必填 integer【总金额】 订单总金额，单位为分
    pub total: u64,
    /// 选填 string(16) 【货币类型】 CNY：人民币，境内商户号仅支持人民币。
    #[builder(default = "\"CNY\".to_string()", setter(into))]
    pub currency: String,
}

// 【支付者】 支付者信息。
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct Payer {
    // 必填 string(128) 【用户标识】 用户在普通商户AppID下的唯一标识。 下单前需获取到用户的OpenID，详见OpenID获取
    pub openid: String,
}

/// 店铺信息
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct StoreInfo {
    /// 必填 string(32) 【门店编号】 商户侧门店编号
    id: String,
    /// 选填 string(256) 门店名称】 商户侧门店名称
    name: Option<String>,
    /// 选填 string(32)【地区编码】 地区编码，详细请见省市区编号对照表。
    area_code: Option<String>,
    /// 选填 string(512) 【详细地址】 详细的商户门店地址
    address: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsapiResult {
    pub app_id: String,
    pub time_stamp: String,
    pub nonce_str: String,
    pub package: String,
    pub sign_type: String,
    pub pay_sign: String,
    pub prepay_id: String,
}

impl JsapiResult {
    pub fn get_sign_str(&self) -> String {
        format!("{}\n{}\n{}\n{}\n", self.app_id, self.time_stamp, self.nonce_str, self.package)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppResult {
    pub appid: String,
    pub partner_id: String,
    pub prepay_id: String,
    pub time_stamp: String,
    pub nonce_str: String,
    pub package_value: String,
    pub sign: String,
}

impl AppResult {
    pub fn get_sign_str(&self) -> String {
        format!("{}\n{}\n{}\n{}\n", self.appid, self.time_stamp, self.nonce_str, self.prepay_id)
    }
}

/// 请求返回体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RPayResponse<T> {
    data: T,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum PayType {
    /// 付款码支付【MICROPAY】付款码支付是用户展示微信钱包内的“刷卡条码/二维码”给商户系统扫描后直接完成支付的模式。主要应用线下面对面收银的场景。
    Micro,
    /// JSAPI支付是用户在微信中打开商户的H5页面，商户在H5页面通过调用微信支付提供的JSAPI接口调起微信支付模块完成支付。应用场景有：
    /// ◆ 用户在微信公众账号内进入商家公众号，打开某个主页面，完成支付
    /// ◆ 用户的好友在朋友圈、聊天窗口等分享商家页面链接，用户点击链接打开商家页面，完成支付
    /// ◆ 将商户页面转换成二维码，用户扫描二维码后在微信浏览器中打开页面后完成支付
    Jsapi,
    /// Native支付是商户系统按微信支付协议生成支付二维码，用户再用微信“扫一扫”完成支付的模式。该模式适用于PC网站支付、实体店单品或订单支付、媒体广告支付等场景。
    Native,
    /// APP支付又称移动端支付，是商户通过在移动端应用APP中集成开放SDK调起微信支付模块完成支付的模式。
    App,
    /// H5支付【MWEB】H5支付主要是在手机、ipad等移动设备中通过浏览器来唤起微信支付的支付产品。
    H5
}

impl Display for PayType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PayType::Micro => write!(f, "MICRO"),
            PayType::Jsapi => write!(f, "JSAPI"),
            PayType::Native => write!(f, "NATIVE"),
            PayType::App => write!(f, "APP"),
            PayType::H5 => write!(f, "H5"),
        }
    }
}

/// 订单详情
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct Detail {
    /// 选填
    /// integer
    /// 【订单原价】 1、商户侧一张小票订单可能被分多次支付，订单原价用于记录整张小票的交易金额。
    /// 2、当订单原价与支付金额不相等，则不享受优惠。
    /// 3、该字段主要用于防止同一张小票分多次支付，以享受多次优惠的情况，正常支付订单不必上传此参数
    pub cost_price: u64,
    /// 选填
    /// string(32)
    /// 【商品小票ID】 商家小票ID
    invoice_id: Option<String>,
    // 必填
    // array[GoodsDetail]
    // 【单品列表】 单品列表信息
    // 条目个数限制：【1，6000】
    goods_detail: Vec<GoodsDetailItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct GoodsDetailItem {
    // 必填
    // string(32)
    // 【商户侧商品编码】 由半角的大小写字母、数字、中划线、下划线中的一种或几种组成。
    pub merchant_goods_id: Option<String>,
    // 选填
    // string(32)
    // 【微信支付商品编码】 微信支付定义的统一商品编号（没有可不传）
    pub wechatpay_goods_id: Option<String>,
    // 选填
    // string(256)
    // 【商品名称】 商品的实际名称
    pub goods_name: String,
    // 必填
    // integer
    // 【商品数量】 用户购买的数量
    pub quantity: u64,
    // 必填
    // integer
    // 【商品单价】 单位为：分。如果商户有优惠，需传输商户优惠后的单价(例如：用户对一笔100元的订单使用了商场发的纸质优惠券100-50，则活动商品的单价应为原单价-50)
    pub unit_price: u64,
}


/// 【结算信息】
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct SettleInfo {
    pub profit_sharing: bool,
}

///【场景信息】
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct SceneInfo {
    /// 必填 string(45) 【用户终端IP】 用户的客户端IP，支持IPv4和IPv6两种格式的IP地址。
    pub payer_client_ip: String,
    /// 选填 string(32) 商户端设备号】 商户端设备号（门店号或收银设备ID）。
    pub device_id: String,
    /// 选填 StoreInfo 【商户门店信息】 商户门店信息
    pub store_info: StoreInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WechatPayDecodeData {
    pub mchid: String,
    pub appid: String,
    pub out_trade_no: String,
    pub transaction_id: String,
    pub trade_type: String,
    pub trade_state: String,
    pub trade_state_desc: String,
    pub bank_type: String,
    pub attach: String,
    pub success_time: String,
    pub payer: PayerInfo,
    pub amount: AmountInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PayerInfo {
    ///【用户标识】 用户在直连商户appid下的唯一标识。
    pub openid: String,
}
impl From<&str> for PayerInfo {
    fn from(value: &str) -> Self {
        Self {
            openid: value.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AmountInfo {
    ///【标价金额】 订单总金额，单位为分。
    pub total: i32,
}

impl From<i32> for AmountInfo {
    fn from(value: i32) -> Self {
        Self { total: value }
    }
}

// 解析微信支付回调参数
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParseEncrypt {
    pub ciphertext: String,
    pub nonce: String,
    pub associated_data: String,
    pub algorithm: String,
    pub original_type: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignData {
    pub app_id: String,
    pub sign_type: String,
    pub package: String,
    pub nonce_str: String,
    pub timestamp: String,
    pub pay_sign: String,
}