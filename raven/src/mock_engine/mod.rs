use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MockDefinition {
    pub mock: HashMap<String, MockBlock>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MockBlock {
    pub r#type: Option<String>,
    pub r#match: MatchBlock,
    pub response: ResponseBlock,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct MatchBlock {
    // HTTP-specific
    /// HTTP method: GET, POST, ...
    pub method: Option<String>,

    /// Path pattern, e.g. "/users/{id}" or regex-like "/users/.*"
    pub path: Option<String>,

    /// Query constraints (exact match per key)
    #[serde(default)]
    pub query: HashMap<String, String>,

    /// Headers constraints (exact match per header)
    #[serde(default)]
    pub header: HashMap<String, String>,

    /// Body match (raw string or JSON fragment) — can be implemented later
    /// For now store as Option<String> and interpret later (template / json / regex)
    pub body: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResponseBlock {
    #[serde(default = "default_status")]
    pub status: u16,

    #[serde(default)]
    pub headers: HashMap<String, String>,

    pub body: Option<String>,

    #[serde(default)]
    pub options: ResponseOptions,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ResponseOptions {
    pub delay: Option<String>,
    pub connection_close: Option<bool>,
}

fn default_status() -> u16 {
    200
}
