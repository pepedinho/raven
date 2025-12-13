use std::collections::HashMap;

use raven_logger::MatchError;
use serde::Deserialize;

use crate::request::ResolvableRequest;

pub mod matcher;

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
    pub path: String,

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

impl MatchBlock {
    pub fn matches<R: ResolvableRequest>(&self, req: &R) -> Result<(), MatchError> {
        // 1) METHOD
        if let Some(ref expected) = self.method {
            let found = req.action().unwrap_or_default();
            if expected != found {
                return Err(MatchError::MethodMismatch {
                    expected: expected.clone(),
                    found: found.to_string(),
                });
            }
        }

        // 2) QUERY
        if !&self.query.is_empty() {
            let req_query = req.query();

            for (key, expected) in &self.query {
                match req_query.get(key) {
                    None => {
                        return Err(MatchError::QueryMismatch {
                            key: key.clone(),
                            expected: expected.clone(),
                            found: None,
                        });
                    }

                    Some(values) if !values.iter().any(|v| v == expected) => {
                        return Err(MatchError::QueryMismatch {
                            key: key.clone(),
                            expected: expected.clone(),
                            found: Some(values.clone()),
                        });
                    }

                    Some(_) => {}
                }
            }
        }

        // 3) HEADERS
        for (key, expected) in &self.header {
            let found = req.headers().get(key);

            if found.map(String::as_str) != Some(expected) {
                return Err(MatchError::HeaderMismatch {
                    key: key.clone(),
                    expected: expected.clone(),
                    found: found.cloned(),
                });
            }
        }

        // 4) BODY
        if let Some(expected) = &self.body
            && expected != req.body()
        {
            return Err(MatchError::BodyMismatch {
                expected: expected.clone(),
                found: req.body().to_string(),
            });
        }

        Ok(())
    }
}
