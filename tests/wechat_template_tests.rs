#[cfg(test)]
mod tests {

    use dotenvy::dotenv;
    use rpay::{pay::config::{WechatV3PayConfig, WechatV3PayConfigBuilder}, template::TemplatesBuilder, RPayResult};

    #[allow(unused)]
    fn get_oepn_id() -> String {
        dotenv().ok();
        let openid = std::env::var("WECHAT_OPEN_ID").expect("WECHAT_OPEN_ID not found");
        return openid;
    }

    // 获取access_token，请通过auth模块获取,将其保存配置文，注意有效期2小时，不要频繁调用
    fn get_access_token() -> String {
        dotenv().ok();
        let openid = std::env::var("WECHAT_ACCESS_TOKEN").expect("WECHAT_ACCESS_TOKEN not found");
        return openid;
    }
    
    // sdk公共参数
    #[allow(unused)]
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

    // 获取行业分类
    #[tokio::test]
    async fn test_get_category() -> RPayResult<()> {
       let resp = TemplatesBuilder::default()
        .access_token(get_access_token())
        .build()?
        .get_category().await?;
        println!("resp => {:?}", resp);
        Ok(())
    }

    // 获取关键词
    #[tokio::test]
    async fn test_get_keywords() -> RPayResult<()> {
       let resp = TemplatesBuilder::default()
        .access_token(get_access_token())
        .build()?
        .get_pub_template_keywords("818".to_string()).await?;
        println!("resp => {:?}", resp);
        Ok(())
    }


}
