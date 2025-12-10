use tokio::net::TcpStream;

pub mod http;

#[async_trait::async_trait]
pub trait Request {
    type Output;

    async fn parse(stream: &mut TcpStream) -> anyhow::Result<Self::Output>;
}
