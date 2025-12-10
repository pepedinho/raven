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

impl ResponseBlock {
    pub fn build(&self) -> String {
        let status_line = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status,
            match self.status {
                200 => "OK",
                201 => "Created",
                400 => "Bad Request",
                404 => "Not found",
                500 => "Internal Server Error",
                _ => "Unknown",
            }
        );

        let mut headers = self.headers.clone();

        if let Some(body) = &self.body {
            headers
                .entry("Content-Length".to_string())
                .or_insert(body.len().to_string());
        } else {
            headers
                .entry("Content-Length".to_string())
                .or_insert("0".to_string());
        }

        let headers_str = headers
            .iter()
            .map(|(k, v)| {
                let clean_v = v.replace("\r", "").replace("\n", "");
                format!("{}: {}\r\n", k, clean_v)
            })
            .collect::<String>();

        let body_str = self.body.clone().unwrap_or_default();

        format!("{}{}\r\n{}", status_line, headers_str, body_str)
    }
}
