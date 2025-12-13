use std::collections::HashMap;

use raven::{mock_engine::MatchBlock, request::http::HttpRequest};
use raven_logger::MatchError;

fn base_request() -> HttpRequest {
    HttpRequest {
        method: "GET".into(),
        path: "/users?id=42".into(),
        version: "HTTP/1.1".into(),
        headers: HashMap::from([("content-type".into(), "application/json".into())]),
        body: "{\"ok\":true}".into(),
    }
}

#[test]
fn method_match_ok() {
    let matcher = MatchBlock {
        method: Some("GET".into()),
        ..Default::default()
    };

    assert!(matcher.matches(&base_request()).is_ok())
}

#[test]
fn method_mismatch() {
    let mut req = base_request();
    req.method = "POST".into();

    let matcher = MatchBlock {
        method: Some("GET".into()),
        ..Default::default()
    };

    let err = matcher.matches(&req).unwrap_err();
    matches!(err, MatchError::MethodMismatch { .. });
}

#[test]
fn query_mismatch() {
    let matcher = MatchBlock {
        query: HashMap::from([("id".into(), "43".into())]),
        ..Default::default()
    };

    assert!(matcher.matches(&base_request()).is_err());
}
