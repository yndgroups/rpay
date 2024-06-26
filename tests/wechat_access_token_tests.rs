#[cfg(test)]
mod tests {

    use dotenvy::dotenv;
    use rpay::{
        auth::access_token::AccessTokenBuilder,
        pay::config::{WechatV3PayConfig, WechatV3PayConfigBuilder},
        RPayResult,
    };
    
    // sdk公共参数
    fn get_sdk() -> RPayResult<WechatV3PayConfig> {
        dotenv().ok();
        let app_id = std::env::var("WECHAT_APP_ID").expect("WECHAT_APP_ID not found");
        let mch_id = std::env::var("WECHAT_MCH_ID").expect("WECHAT_MCH_ID not found");
        let secret = std::env::var("WECHAT_SECRET").expect("WECHAT_SECRET not found");
        let private_key =
            std::env::var("WECHAT_PRIVATE_KEY").expect("WECHAT_PRIVATE_KEY not found");
        let serial_no = std::env::var("WECHAT_SERIAL_NO").expect("WECHAT_SERIAL_NO not found");
        let v3_key = std::env::var("WECHAT_KEY_V3").expect("WECHAT_KEY_V3 not found");
        let notify_url = std::env::var("WECHAT_NOTIFY_URL").expect("WECHAT_NOTIFY_URL not found");
        let sdk = WechatV3PayConfigBuilder::default()
            .app_id(app_id)
            .secret(secret)
            .api_key_v3(v3_key)
            .serial_no(serial_no.to_string())
            .mch_id(mch_id)
            .private_key(private_key)
            .notify_url(notify_url)
            .build()?;
        Ok(sdk)
    }

    /// 获取access_token,将其保存配置文，WECHAT_ACCESS_TOKEN 注意有效期2小时，不要频繁调用
    #[tokio::test]
    async fn test_get_access_token() -> RPayResult<()> {
        let sdk = get_sdk()?;
        let resp = AccessTokenBuilder::default()
            .app_id(sdk.app_id)
            .secret(sdk.secret)
            .build()?
            .request()
            .await?;
        println!("resp => {:?}", resp);
        Ok(())
    }
}
