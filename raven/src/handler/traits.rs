use async_trait::async_trait;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::net::TcpStream;

use crate::mock_engine::MockBlock;

#[async_trait]
pub trait ProtocolHandler: Send + Sync {
    async fn handle(
        &self,
        stream: &mut TcpStream,
        addr: &SocketAddr,
        mock_map: Arc<HashMap<String, MockBlock>>,
    ) -> anyhow::Result<()>;
}
