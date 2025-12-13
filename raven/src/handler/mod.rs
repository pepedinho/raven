pub mod traits;
use async_trait::async_trait;
use raven_logger::{MatchError, Protocol, log, mismatch};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{
    handler::traits::ProtocolHandler,
    mock_engine::{
        MockBlock,
        matcher::{HttpResolver, Resolution, Resolver},
    },
    request::{Request, http::HttpRequest},
};

pub struct HttpHandler;
pub struct WebSocketHandler;
pub struct CustomHandler;

pub fn match_wildcard(request: &str, mock_key: &str) -> Option<HashMap<String, String>> {
    let req_segments: Vec<&str> = request.split("::").collect();
    let mock_segments: Vec<&str> = mock_key.split("::").collect();

    if req_segments.len() != mock_segments.len() {
        return None;
    }

    let mut params = HashMap::new();

    for (req, mock) in req_segments.iter().zip(mock_segments.iter()) {
        if mock.starts_with('{') && mock.ends_with('}') {
            let name = &mock[1..mock.len() - 1];
            params.insert(name.to_string(), (*req).to_string());
            continue;
        }

        if mock != req {
            return None;
        }
    }

    Some(params)
}

#[async_trait]
impl ProtocolHandler for HttpHandler {
    async fn handle(
        &self,
        stream: &mut TcpStream,
        addr: &SocketAddr,
        mock_map: Arc<HashMap<String, MockBlock>>,
    ) -> anyhow::Result<()> {
        let resolver = HttpResolver;
        log!(
            raven_logger::Protocol::Http,
            &addr.to_string(),
            "handling HTTP request..."
        );

        let request = HttpRequest::parse(stream).await?;

        match resolver.resolve(&request, &mock_map)? {
            Resolution::Matched { mock, .. } => {
                HttpHandler::respond(mock, stream).await?;
            }
            Resolution::Mismatch { diagnostics } => {
                HttpHandler::mismatch(stream, addr, &diagnostics).await?;
            }
            Resolution::NotFound => {
                HttpHandler::no_mock(stream, &request, addr).await?;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl ProtocolHandler for WebSocketHandler {
    async fn handle(
        &self,
        _stream: &mut TcpStream,
        addr: &SocketAddr,
        _mock_map: Arc<HashMap<String, MockBlock>>,
    ) -> anyhow::Result<()> {
        log!(
            raven_logger::Protocol::WebSocket,
            &addr.to_string(),
            "handling websocket request..."
        );
        Ok(())
    }
}

impl HttpHandler {
    async fn respond(mock: &MockBlock, stream: &mut TcpStream) -> anyhow::Result<()> {
        let response = mock.response.build();

        if let Some(delay) = &mock.response.options.delay {
            let duration = humantime::parse_duration(delay)?;
            tokio::time::sleep(duration).await;
        }
        stream.write_all(response.as_bytes()).await?;
        stream.flush().await?;
        if mock.response.options.connection_close.unwrap_or(false) {
            stream.shutdown().await?;
        }
        Ok(())
    }

    async fn no_mock(
        stream: &mut TcpStream,
        request: &HttpRequest,
        addr: &SocketAddr,
    ) -> anyhow::Result<()> {
        let status_line = "HTTP/1.1 404 Not Found\r\n";
        let body = "404 Not Found";
        let headers = format!(
            "Content-Length: {}\r\nContent-Type: text/plain\r\n",
            body.len()
        );

        let response = format!("{}{}\r\n{}", status_line, headers, body);
        stream.write_all(response.as_bytes()).await?;
        log!(
            raven_logger::Protocol::Http,
            &addr.to_string(),
            "No mock matched for path {}",
            request.path
        );
        Ok(())
    }

    async fn mismatch(
        stream: &mut TcpStream,
        addr: &SocketAddr,
        err: &MatchError,
    ) -> anyhow::Result<()> {
        let msg = match err {
            MatchError::MethodMismatch { expected, found } => {
                format!("Method mismatch: expected '{}', got '{}'", expected, found)
            }
            MatchError::QueryMismatch {
                key,
                expected,
                found,
            } => {
                format!(
                    "Query mismatch on '{}': expected '{}', got '{:?}'",
                    key, expected, found
                )
            }
            MatchError::HeaderMismatch {
                key,
                expected,
                found,
            } => {
                format!(
                    "Header mismatch on '{}': expected '{}', got '{:?}'",
                    key, expected, found
                )
            }
            MatchError::BodyMismatch { expected, found } => {
                format!("Body mismatch: expected '{}', got '{}'", expected, found)
            }
        };

        mismatch!(Protocol::Http, &addr.to_string(), err);

        let response = format!(
            "HTTP/1.1 422 Unprocessable Entity\r\n\
             Content-Type: text/plain\r\n\
             Content-Length: {}\r\n\
             \r\n{}",
            msg.len(),
            msg
        );

        stream.write_all(response.as_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
impl ProtocolHandler for CustomHandler {
    async fn handle(
        &self,
        _stream: &mut TcpStream,
        addr: &SocketAddr,
        _mock_map: Arc<HashMap<String, MockBlock>>,
    ) -> anyhow::Result<()> {
        log!(
            raven_logger::Protocol::Custom,
            &addr.to_string(),
            "handling custom protocol request..."
        );
        Ok(())
    }
}
