# RPay 基于rust语言的支付工具库

## 微信、支付宝、QQ、通联支付、拉卡拉、PayPal的支付SDK

-- 代码结构
    |-- crates
        |-- .github       github
        |-- .vscode       vscode配置
        |-- lib-alipay    支付宝
        |-- lib-allinpay  通联支付
        |-- lib-apple     苹果支付
        |-- lib-lakala     拉卡拉
        |-- lib-paypal     PayPal
        |-- lib-qq         QQ
        |-- lib-wechat     微信         
    |-- .gitignore         忽略提交文件
    |-- Cargo.toml          cargo 配置文件
    |-- README.md           说明文档

## 测试配置请照着.env.tpl格式创建.env文件并填写支付信息内容
```
cargo test --test wechat_tests
```
