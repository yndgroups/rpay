use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    core::request::Request, RPayResult
};
use super::config::WechatV3PayConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub struct Refunds {
    /// 选填 string(32) 微信支付订单号】 原支付交易对应的微信订单号，与out_trade_no二选一
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// 必填 string(64)【商户退款单号】 商户系统内部的退款单号，商户系统内部唯一，只能是数字、大小写字母_-|*@ ，同一退款单号多次请求只退一笔。
    #[builder(setter(into))]
    pub out_trade_no: String,
    //  选填 string(80) 【退款原因】 若商户传入，会在下发给用户的退款消息中体现退款原因
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 选填 string(256)【退款结果回调url】 异步接收微信支付退款结果通知的回调地址，通知url必须为外网可访问的url，不能携带参数。 如果参数中传了notify_url，则商户平台上配置的回调地址将不会生效，优先回调当前传的这个地址。
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    ///  选填 string 退款资金来源】 若传递此参数则使用对应的资金账户退款，否则默认使用未结算资金退款（仅对老资金流商户适用）
    /// 可选取值：AVAILABLE: 仅对老资金流商户适用，指定从可用余额账户出资
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funds_account: Option<String>,
    /// 必填 AmountReq【金额信息】 订单金额信息
    #[builder(setter)]
    pub amount: AmountReq,
    /// goods_detail 选填 array[GoodsDetail] 【退款商品】 指定商品退款需要传此参数，其他场景无需传递
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_detail: Option<Vec<GoodsDetail>>,
}

/// 实现方法
impl Refunds {
    pub async fn pay(&mut self, wechat_sdk: WechatV3PayConfig) -> RPayResult<RefundResponse> {
        self.notify_url = wechat_sdk.refund_notify_url.clone();
        let json_body = serde_json::to_string(self).unwrap();
        Request::build_pay_request::<RefundResponse>(wechat_sdk,crate::common::HttpMethod::POST, "/v3/refund/domestic/refunds", json_body).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct GoodsDetail {
    /// 必填 string(32)【商户侧商品编码】 商品编码，由半角的大小写字母、数字、中划线、下划线中的一种或几种组成。
    #[builder(setter)]
    pub merchant_goods_id: String,
    /// 选填 string(32)【微信侧商品编码】 微信支付定义的统一商品编号（没有可不传）
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechatpay_goods_id: Option<String>,
    ///  选填 string(256)【商品名称】 商品的实际名称
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_name: Option<String>,
    /// unit_price 必填 integer【商品单价】 商品单价金额，单位为分
    #[builder(setter)]
    pub unit_price: u64,
    ///  必填 integer【商品退款金额】 商品退款金额，单位为分
    #[builder(setter)]
    pub refund_amount: u64,
    ///  必填 integer 商品退货数量】 对应商品的退货数量
    #[builder(setter)]
    pub refund_quantity: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct AmountReq {
    /// 必填 integer 【退款金额】 退款金额，单位为分，只能为整数，不能超过原订单支付金额。
    #[builder(setter(into))]
    pub refund: i64,
    /// 选填 array[FundsFromItem] 【退款出资账户及金额】 退款需要从指定账户出资时，传递此参数指定出资金额（币种的最小单位，只能为整数）。
    // 同时指定多个账户出资退款的使用场景需要满足以下条件：1、未开通退款支出分离产品功能；2、订单属于分账订单，且分账处于待分账或分账中状态。
    // 参数传递需要满足条件：1、基本账户可用余额出资金额与基本账户不可用余额出资金额之和等于退款金额；2、账户类型不能重复。
    // 上述任一条件不满足将返回错误
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<FundsFromItem>>,
    /// 必填 integer 【原订单金额】 原支付交易的订单总金额，单位为分，只能为整数。
    #[builder(setter(into))]
    pub total: i64,
    /// 必填 string(16) 退款币种】 符合ISO 4217标准的三位字母代码，目前只支持人民币：
    #[builder(default = "\"CNY\".to_string()", setter(into))]
    pub currency: String,
}

/// 账户出资金额
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct FundsFromItem {
    /// 必填 string 【出资账户类型】 出资账户类型 可选取值：AVAILABLE: 可用余额 UNAVAILABLE: 不可用余额
    #[builder(setter(into))]
    pub account: String,
    /// amount 必填 integer 出资金额】 对应账户出资金额，单位为分
    #[builder(setter(into))]
    pub amount: u64,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RefundResponse {
    /// 退款编号
    pub refund_id: String,
    /// 商户订单编号
    pub out_trade_no: String,
    /// 微信交易编号
    pub transaction_id: String,
    /// 退款单号
    pub out_refund_no: String,
    /// 退款渠道 枚举值：
    ///  ORIGINAL—原路退款
    ///  BALANCE—退回到余额
    ///  OTHER_BALANCE—原账户异常退到其他余额账户
    ///  OTHER_BANKCARD—原银行卡异常退到其他银行卡
    pub channel: String,
    ///  退款入账账户
    /// 描述：
    ///  取当前退款单的退款入账方，有以下几种情况：
    ///  1）退回银行卡：{银行名称}{卡类型}{卡尾号}
    ///  2）退回支付用户零钱:支付用户零钱
    ///  3）退还商户:商户基本账户商户结算银行账户
    ///  4）退回支付用户零钱通:支付用户零钱通
    pub user_received_account: String,
    ///  退款成功时间
    pub success_time: Option<String>,
    ///  退款创建时间
    pub create_time: String,
    ///  退款状态
    ///  退款到银行发现用户的卡作废或者冻结了，导致原路退款银行卡失败，可前往商户平台（pay.weixin.qq.com）-交易中心，手动处理此笔退款。
    ///  枚举值：
    ///  SUCCESS：退款成功
    ///  CLOSED：退款关闭
    ///  PROCESSING：退款处理中
    ///  ABNORMAL：退款异常
    pub status: String,
    /// 资金账户 退款所使用资金对应的资金账户类型
    /// 枚举值：
    ///  UNSETTLED : 未结算资金
    ///  AVAILABLE : 可用余额
    ///  UNAVAILABLE : 不可用余额
    ///  OPERATION : 运营户
    ///  BASIC : 基本账户（含可用余额和不可用余额）
    pub funds_account: Option<String>,
    /// 金额信息
    pub amount: RefundAmount,
    /// 优惠退款信息
    pub promotion_detail: Option<Vec<RefundPromotionDetail>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefundAmount {
    /// 退款金额，单位为分。 退款金额，币种的最小单位，只能为整数，不能超过原订单支付金额。
    pub refund: i64,
    /// 原支付交易的订单总金额，币种的最小单位，只能为整数。
    pub total: i64,
    /// 用户实际支付金额，单位为分，只能为整数，详见支付金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_total: Option<i64>,
    /// 退款给用户的金额，不包含所有优惠券金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_refund: Option<i64>,
    /// 币类型, CNY：人民币，境内商户号仅支持人民币。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}


/// 数据返回
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefundPromotionDetail {
    /// 券ID
    pub promotion_id: String,
    /// 优惠范围 GLOBAL：全场代金券 SINGLE：单品优惠
    pub scope: Option<String>,
    /// COUPON：代金券，需要走结算资金的充值型代金券 *  DISCOUNT：优惠券，不走结算资金的免充值型优惠券
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// 优惠券面额
    pub amount: i64,
    /// 优惠退款金额<=退款金额，退款金额-代金券或立减优惠退款金额为用户支付的现金，说明详见代金券或立减优惠，单位为分
    pub refund_amount: i64,
    /// 单品列表
    pub goods_detail: Option<GoodsDetail>,
}