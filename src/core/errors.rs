use std::{num::ParseIntError, string::FromUtf8Error};

use base64::DecodeError;

use crate::{model::{AmountBuilderError, PayerBuilderError}, pay::{app, config::WechatV3PayConfigBuilderError, h5, jsapi::JsApiPayBuilderError, native, parse_encrypt::ParseEncryptBuilderError, pay_info}};

#[derive(Debug, thiserror::Error)]
pub enum RPayError {
    /// io异常
    #[error("io异常: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("serde_json解析异常: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("utf8转换异常: {0}")]
    FromUtf8Error(#[from] FromUtf8Error),

    #[error("reqwest异常: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("格式转换失败: {0}")]
    ParseIntError(#[from] ParseIntError),

    #[error("错误编码:{0},错误原因: {1}")]
    Error(String, String),

    #[error("错误原因:{0}")]
    ErrorWithMsg(String),
    
    #[error("错误原因:{0}")]
    DecodeError(#[from] DecodeError),

    #[error("reqwest请求头异常: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error("x509解析异常: {0}")]
    X509Error(#[from] x509_parser::nom::Err<x509_parser::prelude::PEMError>),

    #[error("证书异常异常: {0}")]
    PEMError(#[from] x509_parser::nom::Err<x509_parser::prelude::X509Error>),
    
    #[error("支付SDK初始化异常: {0}")]
    WechatV3PayConfigBuilderError(#[from] WechatV3PayConfigBuilderError),

    #[error("jsapi支付参数构建异常: {0}")]
    JsApiPayBuilderError(#[from] JsApiPayBuilderError),

    #[error("h5支付参数构建异常: {0}")]
    H5PayBuilderError(#[from] h5::H5PayBuilderError),

    #[error("app支付参数构建异常: {0}")]
    AppPayBuilderError(#[from] app::AppPayBuilderError),

    #[error("native支付参数构建异常: {0}")]
    NativePayBuilderError(#[from] native::NativePayBuilderError),

    #[error("下单支付者参数: {0}")]
    PayerBuilderError(#[from] PayerBuilderError),

    #[error("h5支付场景参数构建异常: {0}")]
    H5ReqSceneInfoBuilderError(#[from] h5::H5ReqSceneInfoBuilderError),
    
    #[error("h5支付参数构建异常: {0}")]
    H5InfoBuilderError(#[from] h5::H5InfoBuilderError),
    
    #[error("下单支付金额参数: {0}")]
    AmountBuilderError(#[from] AmountBuilderError),

    #[error("支付签名参数异常: {0}")]
    PayInfoBuilderError(#[from] pay_info::PayInfoBuilderError),

    #[error("支付签名参数异常: {0}")]
    ParseEncryptBuilderError(#[from] ParseEncryptBuilderError),
    
}