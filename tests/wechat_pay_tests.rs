#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use dotenvy::dotenv;
    use rpay::{model::{AmountBuilder, PayType, PayerBuilder}, pay::{
            app::AppPayBuilder, config::{WechatV3PayConfig, WechatV3PayConfigBuilder}, h5::H5PayBuilder, jsapi::JsApiPayBuilder, native::NativePayBuilder, parse_encrypt::ParseEncryptBuilder, pay_info::PayInfoBuilder
        }, RPayResult
    };

    fn get_oepn_id() -> String {
        let openid = std::env::var("WECHAT_OPEN_ID").expect("WECHAT_OPEN_ID not found");
        return openid;
    }
    
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
   
    /// 测试解密
    #[tokio::test]
    async fn test_pay_info() -> RPayResult<()> {
        let sdk = get_sdk()?;
        let resp = PayInfoBuilder::default()
            .prepay_id("wx14154500982430c7bee6ccd124b7400000") // 这里注意动态获取的参数
            .pay_type(PayType::Jsapi)
            .build()?.signature(sdk).await?;
        println!("resp => {:?}", resp);
        Ok(())
    }

    /// 测试获取支付信息用于支付
    #[tokio::test]
    async fn decrypt_pay_data() -> RPayResult<()> {
        let sdk = get_sdk()?;
        let resp = ParseEncryptBuilder::default()
            .algorithm("AEAD_AES_256_GCM")
            .associated_data("transaction")
            .original_type("transaction")
            .ciphertext("+iVAZMU1iCmdiYuJbQmAxBy1QU4D+EO2kqPiYx3UvHwGWUbucacP4eKxuI3+WBjOP59QU1IW+hRwMiU+9GSR6DFp5deB2aCvIWuj1EjL1uVWg4UvUqMm1YmO0ROE5vhSK4wWe+tqiB+j+EMho5Xm2M03nQYt5wdQiXLUHDkzC9b8iM9Cjy6sQYyejc4zCZN7TEbIt3BDqBgHKyUzLzA01CQ2f2BR5NqJbXhl/0OSNz2LMlmOKOsGnuD86Pb/w0g2Zz6UzkduMqnJQUFaa3QhOD+dKJI00KF8gqzNU7eCTHqbF7qaoxaWJ80B4q3BGodqS6xH0GkT++SjpGDd/gHd+T/l/7Mw/aPnt3KmrRCbZJ6k/aBHAbZkvfr+5h0ij/MkF97yDBIcWbMbJB0buL6QXtwII/pXuXzLTIlctdg1jDQUAsK0xHm5FVLbKDQq8Kbb+70WeZeb+C5iYfGBxEfiBo0s71IRyhZx6Jfyg5yNUd/hP9kwnpDZI/uu/xyi7GmNGCVXMg7E9eSNukRmjQl87+Svg3Q7tWVJEmECwt6T7ks0jTkcqihy8EptJhBtomETbJu9Ia/HsY4Iw0jDqBG9")
            .nonce("Sum4M3feZ35C")
            .build()?.parse(sdk).await?;
        println!("{:?}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_build_jsapi_pay() -> RPayResult<()> {
        // sdk
        let sdk = get_sdk()?;
        // 创建支付请求参数
        let resp = JsApiPayBuilder::default()
            .description("测试支付")
            .payer(
                PayerBuilder::default()
                    .openid(get_oepn_id())
                    .build()?,
            )
            .out_trade_no("wx1123123232432341223")
            // .notify_url(sdk.notify_url)
            .amount(
                AmountBuilder::default()
                    .total(100)
                    .build()?,
            )
            .build()?.pay(sdk).await?;
        // 返回结果
        println!("test_build_jsapi_pay resp => {:?}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_h5() -> RPayResult<()> {
        // sdk
        let sdk = get_sdk()?;
        // 创建支付请求参数
        let resp = H5PayBuilder::default()
            .description("测试H5支付")
            .out_trade_no("wx1123123232432341223")
            // .notify_url(sdk.notify_url)
            .amount(
                AmountBuilder::default()
                    .total(100)
                    .build()?,
            )
            .build()?.pay(sdk).await?;
        // 返回结果
        println!("test_build_jsapi_pay resp => {:?}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_app() -> RPayResult<()> {
        // sdk
        let sdk = get_sdk()?;
        // 创建支付请求参数
        let resp = AppPayBuilder::default()
            .description("测试App支付")
            .out_trade_no("wx1123123232432341223")
            // .notify_url(sdk.notify_url)
            .amount(
                AmountBuilder::default()
                    .total(100)
                    .build()?,
            )
            .build()?.pay(sdk).await?;
        // 返回结果
        println!("test_build_jsapi_pay resp => {:?}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_native() -> RPayResult<()> {
        // sdk
        let sdk = get_sdk()?;
        // 创建支付请求参数
        let resp = NativePayBuilder::default()
            .description("测试原生支付")
            .out_trade_no("wx1123123232432341223")
            // .notify_url(sdk.notify_url)
            .amount(
                AmountBuilder::default()
                    .total(100)
                    .build()?,
            )
            .build()?.pay(sdk).await?;
        // 返回结果
        println!("test_build_jsapi_pay resp => {:?}", resp);
        Ok(())
    }
}
