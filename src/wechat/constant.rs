pub static ACCEPT: &str = "Accept";
pub static AUTHORIZATION: &str = "Authorization";
pub static TIMEOUT: u64 = 30;

// wechat v3 地址
pub static MCH_HOST: &str = "https://api.mch.weixin.qq.com";
pub static V3_PAY_TRANSACTIONS_JS_API: &str = "/v3/pay/transactions/jsapi";
pub static V3_PAY_TRANSACTIONS_H5: &str = "/v3/pay/transactions/h5";
pub static V3_PAY_TRANSACTIONS_APP: &str = "/v3/pay/transactions/app";
pub static V3_PAY_TRANSACTIONS_NATIVE: &str = "/v3/pay/transactions/native";
pub static V3_REFUND_DOMESTIC_REFUNDS: &str = "/v3/refund/domestic/refunds";


// auth
pub static CGI_BIN_TOKEN_WECHAT_HOST: &str = "https://api.weixin.qq.com/cgi-bin/token";

// 小城模版管理 template
pub static NEW_TMPL_HOST: &str =  "https://api.weixin.qq.com/wxaapi/newtmpl";

// message
// 公众号
// pub static MESSAGE_HOST: &str = "https://api.weixin.qq.com/cgi-bin/message";
// pub static MESSAGE_SEND: &str = "/subscribe/bizsend?access_token=";
// 小程序
pub static MESSAGE_HOST: &str = "https://api.weixin.qq.com/cgi-bin/message";
pub static MESSAGE_SEND: &str = "/subscribe/send?access_token=";