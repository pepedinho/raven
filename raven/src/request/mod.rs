use std::collections::HashMap;

use tokio::net::TcpStream;

pub mod http;

#[async_trait::async_trait]
pub trait Request {
    type Output;

    async fn parse(stream: &mut TcpStream) -> anyhow::Result<Self::Output>;
}

pub trait ResolvableRequest {
    fn action(&self) -> Option<&str>;
    fn headers(&self) -> &HashMap<String, String>;
    fn body(&self) -> &str;

    fn route(&self) -> &str;

    fn query(&self) -> &HashMap<String, Vec<String>>;
}
