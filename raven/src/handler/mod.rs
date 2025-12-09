pub mod traits;
use async_trait::async_trait;
use raven_logger::log;
use std::net::SocketAddr;
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::handler::traits::ProtocolHandler;

pub struct HttpHandler;
pub struct WebSocketHandler;
pub struct CustomHandler;

#[async_trait]
impl ProtocolHandler for HttpHandler {
    async fn handle(&self, stream: &mut TcpStream, addr: &SocketAddr) -> anyhow::Result<()> {
        log!(
            raven_logger::Protocol::Http,
            &addr.to_string(),
            "handling HTTP request..."
        );

        let response = b"HTTP/1.1 200 OK\r\n\
                         Content-Type: text/plain\r\n\
                         Content-Length: 5\r\n\
                         Connection: close\r\n\
                         \r\n\
                         Hello";

        stream.write_all(response).await?;
        stream.flush().await?;
        Ok(())
    }
}

#[async_trait]
impl ProtocolHandler for WebSocketHandler {
    async fn handle(&self, _stream: &mut TcpStream, addr: &SocketAddr) -> anyhow::Result<()> {
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
    async fn handle(&self, _stream: &mut TcpStream, addr: &SocketAddr) -> anyhow::Result<()> {
        log!(
            raven_logger::Protocol::Custom,
            &addr.to_string(),
            "handling custom protocol request..."
        );
        Ok(())
    }
}
