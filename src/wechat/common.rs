use reqwest::{Client, RequestBuilder};
use std::fmt::{Display, Formatter};

pub trait IntoRequest {
    fn into_request(self, client: Client) -> RequestBuilder;
    fn get_body(&self) -> String;
    fn get_api(&self) -> String;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl Default for HttpMethod {
    fn default() -> Self {
        HttpMethod::GET
    }
}

impl Display for HttpMethod {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(fmt, "GET"),
            HttpMethod::POST => write!(fmt, "POST"),
            HttpMethod::PUT => write!(fmt, "PUT"),
            HttpMethod::DELETE => write!(fmt, "DELETE"),
            HttpMethod::PATCH => write!(fmt, "PATCH"),
        }
    }
}
