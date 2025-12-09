use async_trait::async_trait;
use std::net::SocketAddr;
use tokio::net::TcpStream;

#[async_trait]
pub trait ProtocolHandler: Send + Sync {
    async fn handle(&self, stream: &mut TcpStream, addr: &SocketAddr) -> anyhow::Result<()>;
}
