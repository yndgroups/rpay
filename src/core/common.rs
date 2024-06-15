use crate::RPayError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub trait RPayResponse: DeserializeOwned {}
pub type RPayResult<T> = std::result::Result<T, RPayError>;