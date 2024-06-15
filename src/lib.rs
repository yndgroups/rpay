mod core;

#[cfg(feature = "alipay")]
mod alipay;

#[cfg(feature = "allinpay")]
mod allinpay;

#[cfg(feature = "apple")]
mod  apple;

#[cfg(feature = "lakala")]
mod lakala;

#[cfg(feature = "paypal")]
mod paypal;

#[cfg(feature = "qq")]
mod qq;

#[cfg(feature = "wechat")]
mod wechat;

#[cfg(feature = "wechat")]
pub use wechat::*;

pub use core::errors::RPayError;
pub type RPayResult<T, E = RPayError> = Result<T, E>;
pub use reqwest::multipart::{Form, Part};