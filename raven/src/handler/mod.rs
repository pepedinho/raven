pub mod traits;
use async_trait::async_trait;
use raven_logger::log;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{
    handler::traits::ProtocolHandler,
    mock_engine::MockBlock,
    request::{Request, http::HttpRequest},
};

pub struct HttpHandler;
pub struct WebSocketHandler;
pub struct CustomHandler;

#[async_trait]
impl ProtocolHandler for HttpHandler {
    async fn handle(
        &self,
        stream: &mut TcpStream,
        addr: &SocketAddr,
        mock_map: Arc<HashMap<String, MockBlock>>,
    ) -> anyhow::Result<()> {
        log!(
            raven_logger::Protocol::Http,
            &addr.to_string(),
            "handling HTTP request..."
        );

        let request = HttpRequest::parse(stream).await?;

        let path = request
            .path
            .trim_start_matches('/')
            .trim_end_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("::");

        if let Some(mock) = mock_map.get(&path) {
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
        } else {
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
        }

        // println!("debug: from ({}) to ({})", request.path, path);

        // println!("request: {:#?}", request);

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
