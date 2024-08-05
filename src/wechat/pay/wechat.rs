use uuid::Uuid;

use crate::model::{SignData, WechatPay};

pub trait WechatPayTrait {
    fn app_id(&self) -> String;
    fn mch_id(&self) -> String;
    fn private_key(&self) -> String;
    fn serial_no(&self) -> String;
    fn v3_key(&self) -> String;
    fn notify_url(&self) -> String;
    fn domain(&self) -> String;
    fn rsa_sign(&self, content: impl AsRef<str>) -> String;
    fn now_timestamp(&self) -> String {
        chrono::Local::now().timestamp().to_string()
    }
    fn nonce_str(&self) -> String {
        Uuid::new_v4().to_string().replace("-", "").to_uppercase()
    }

    fn mut_sign_data<S>(&self, prefix: S, prepay_id: S) -> SignData
    where
        S: AsRef<str>,
    {
        let app_id = self.app_id();
        let now_time = self.now_timestamp();
        let nonce_str = self.nonce_str();
        let ext_str = format!(
            "{prefix}{prepay_id}",
            prefix = prefix.as_ref(),
            prepay_id = prepay_id.as_ref()
        );
        let signed_str = self.rsa_sign(format!("{app_id}\n{now_time}\n{nonce_str}\n{ext_str}\n"));
        SignData {
            app_id,
            sign_type: "RSA".into(),
            package: ext_str,
            nonce_str,
            timestamp: now_time,
            pay_sign: signed_str,
        }
    }
}


impl WechatPay {
    pub fn new<S: AsRef<str>>(
        app_id: S,
        mch_id: S,
        private_key: S,
        serial_no: S,
        v3_key: S,
        notify_url: S,
    ) -> Self {
        Self {
            app_id: app_id.as_ref().to_string(),
            mch_id: mch_id.as_ref().to_string(),
            private_key: private_key.as_ref().to_string(),
            serial_no: serial_no.as_ref().to_string(),
            v3_key: v3_key.as_ref().to_string(),
            notify_url: notify_url.as_ref().to_string(),
            domain: "https://api.mch.weixin.qq.com".to_string(),
        }
    }

    pub fn from_env() -> Self {
        // let secret = std::env::var("WECHAT_SECRET").expect("WECHAT_SECRET not found");
        let app_id = std::env::var("WECHAT_APP_ID").expect("WECHAT_APP_ID not found");
        let mch_id = std::env::var("WECHAT_MCH_ID").expect("WECHAT_MCH_ID not found");
        let private_key =
            std::env::var("WECHAT_PRIVATE_KEY").expect("WECHAT_PRIVATE_KEY not found");
        let serial_no = std::env::var("WECHAT_SERIAL_NO").expect("WECHAT_SERIAL_NO not found");
        let v3_key = std::env::var("WECHAT_KEY_V3").expect("WECHAT_KEY_V3 not found");
        let notify_url = std::env::var("WECHAT_NOTIFY_URL").expect("WECHAT_NOTIFY_URL not found");
        Self::new(app_id, mch_id, private_key, serial_no, v3_key, notify_url)
    }
}