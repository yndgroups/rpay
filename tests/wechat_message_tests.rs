#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use dotenvy::dotenv;
    use rpay::{
        message::{message::{DataItem, MessageBuilder}, CustomerServiceBuilder, Link, Miniprogrampage, MsgType, Text}, pay::config::{WechatV3PayConfig, WechatV3PayConfigBuilder}, RPayError, RPayResult
    };

    // 获取用的的openid
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

    /// 测试 - 发送客服 - 卡片消息
    #[tokio::test]
    async fn test_kf_send_link_msg() -> RPayResult<()> {
       let resp =  CustomerServiceBuilder::default()
        .access_token(get_access_token())
        .touser(get_oepn_id())
        .msg_type(MsgType::Link)
        .link(Some(Link{
            title: Some("新订单来了".to_string()),
             description: Some("五香辣椒248g两包<br/>麻辣498一包".to_string()),
             url: Some("https://www.baidu.com".to_string()),
             thumb_url: Some("https://www.baidu.com".to_string()),
        }))
        .build()
        .unwrap()
        .send()
        .await?;
        println!("resp => {:?}", resp);
        Ok(())
    }

      /// 测试 - 发送客服 - 卡片消息
      #[tokio::test]
      async fn test_kf_card_send() -> RPayResult<()> {
         let resp =  CustomerServiceBuilder::default()
          .access_token(get_access_token())
          .touser(get_oepn_id())
          .msg_type(MsgType::Miniprogrampage)
          .miniprogrampage(Some(Miniprogrampage{
            title: Some("测试玩一把".to_string()),
             pagepath: Some("pages/index/index".to_string()),
             thumb_media_id: None,
          }))
          .build()
          .unwrap()
          .send()
          .await?;
          println!("resp => {:?}", resp);
          Ok(())
      }
  

    /// 测试 - 发送客服 - 文本消息
    #[tokio::test]
    async fn test_kf_text_send() -> RPayResult<()> {
       let resp =  CustomerServiceBuilder::default()
        .access_token(get_access_token())
        .touser(get_oepn_id())
        .msg_type(MsgType::Text)
        .text(Some(Text {
            content: "我是后台接口主动发起的信息，王先生起来接单了，以后就用这个给你推送订单信息哈".to_string(),
        }))
        .build()
        .unwrap()
        .send()
        .await?;
        println!("resp => {:?}", resp);
        Ok(())
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

    /// 测试发送发货消息
    #[tokio::test]
    async fn test_message_send() -> RPayResult<()> {
        let mut data = HashMap::new();
        data.insert(
            "character_string5".to_string(),
            DataItem {
                value: "快递单号".to_string(),
            },
        );
        data.insert(
            "character_string1".to_string(),
            DataItem {
                value: "订单编号".to_string(),
            },
        );
        data.insert(
            "thing20".to_string(),
            DataItem {
                value: "快递公司".to_string(),
            },
        );
        data.insert(
            "thing21".to_string(),
            DataItem {
                value: "物品名称".to_string(),
            },
        );
        data.insert(
            "thing25".to_string(),
            DataItem {
                value: "发货门店".to_string(),
            },
        );
        let resp = MessageBuilder::default()
            .access_token(get_access_token())
            .touser(get_oepn_id())
            .template_id("aZHCMNQEVOQjF5SSYuPI_eV0tuBj7R7-UQYuQ9FQWx4")
            .data(data)
            .build().map_err(|err|RPayError::ErrorWithMsg(err.to_string()))?
            .send().await?;
        println!("resp => {:?}", resp);
        Ok(())
    }

    /// 测试下单模版消息
    #[tokio::test]
    async fn test_message_send_create_order() -> RPayResult<()> {
        let mut data = HashMap::new();
        data.insert(
            "thing1".to_string(),
            DataItem {
                value: "大洋芋".to_string(), // 商品名称
            },
        );
        data.insert(
            "amount2".to_string(),
            DataItem {
                value: "18.00".to_string(), // 商品金额
            },
        );
        data.insert(
            "number3".to_string(),
            DataItem {
                value: "1".to_string(), // 商品数量
            },
        );
        data.insert(
            "time4".to_string(),
            DataItem {
                value: "2024-06-21 14:16:33".to_string(), // 下单时间格式必须正确(2024-06-21 14:16:33)
            },
        );
        let resp = MessageBuilder::default()
            .access_token(get_access_token())
            .touser(get_oepn_id())
            .template_id("rc-r-FZ6gwiq2tvkWwJeFckWRkU-RmReKyeFfkfFQLs")
            .data(data)
            .page("pages/order/index".to_string())
            .build().map_err(|err|RPayError::ErrorWithMsg(err.to_string()))?
            .send().await?;
        println!("resp => {:?}", resp);
        Ok(())
    }

     /// 测试下单模版消息
     #[tokio::test]
     async fn test_message_send_have_order() -> RPayResult<()> {
         let mut data = HashMap::new();
         data.insert(
             "name7".to_string(),
             DataItem {
                 value: "下单人".to_string(),
             },
         );
         data.insert(
             "thing9".to_string(),
             DataItem {
                 value: "收货地址".to_string(),
             },
         );
         data.insert(
             "thing5".to_string(),
             DataItem {
                 value: "备注信息".to_string(),
             },
         );
         data.insert(
             "thing6".to_string(),
             DataItem {
                 value: "商品名称".to_string(),
             },
         );
         data.insert(
            "number12".to_string(),
            DataItem {
                value: "2".to_string(),
            },
        );
         let resp = MessageBuilder::default()
             .access_token(get_access_token())
             .touser(get_oepn_id())
             .template_id("rc-r-FZ6gwiq2tvkWwJeFckWRkU-RmReKyeFfkfFQLs")
             .data(data)
             .build()
             .map_err(|err|RPayError::ErrorWithMsg(err.to_string()))?
             .send().await?;
         println!("resp => {:?}", resp);
         Ok(())
     }
 
}
