use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    #[serde(alias = "get")]
    Get,
    #[serde(alias = "post")]
    Post,
}

#[derive(Debug, Deserialize)]
pub struct HttpCommand {
    pub url: String,
    pub method: HttpMethod,
    pub env: Option<HashMap<String, String>>,
    pub body: Option<HashMap<String, String>>,
    pub headers: Option<HashMap<String, String>>,
    pub then: String,
}
