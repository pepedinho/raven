use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use raven_logger::slog;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

use crate::{
    cli::Cli,
    handler::{CustomHandler, HttpHandler, WebSocketHandler, traits::ProtocolHandler},
    mock_engine::MockBlock,
    resolver::Protocol,
};

#[async_trait]
pub trait Listener: Send + Sync + 'static {
    async fn listen_on(self: Arc<Self>, addr: &str) -> anyhow::Result<()>;
    async fn handle_new_connexion(
        self: Arc<Self>,
        stream: &mut TcpStream,
        addr: &SocketAddr,
    ) -> anyhow::Result<()>;
}

#[derive(Clone)]
pub struct Client {}

/// Perch is concretly the Listener we use to Listen clients requests
pub struct Perch {
    port: u16,
    clients: HashMap<String, Client>,
    handlers: HashMap<Protocol, Arc<dyn ProtocolHandler>>,
    pub mock_map: Arc<HashMap<String, MockBlock>>,
}

impl Perch {
    pub fn new(cli: &Cli) -> anyhow::Result<Self> {
        let mut handlers: HashMap<Protocol, Arc<dyn ProtocolHandler>> = HashMap::new();

        handlers.insert(Protocol::Http, Arc::new(HttpHandler));
        handlers.insert(Protocol::WebSocket, Arc::new(WebSocketHandler));
        handlers.insert(Protocol::Custom, Arc::new(CustomHandler));

        let mock_map = match (&cli.mock, &cli.mocks) {
            (Some(file), _) => Arc::new(Cli::load_mock_file(file)?),
            (_, Some(dir)) => Arc::new(Cli::load_mock_dir(dir, dir)?),
            _ => {
                return Err(anyhow::anyhow!(
                    "You must provide etiher --mock <file> or --mocks <dir>"
                ));
            }
        };

        Ok(Self {
            port: cli.port,
            clients: HashMap::new(),
            mock_map,
            handlers,
        })
    }

    pub fn check_client(&self, addr: &str) -> Option<Client> {
        self.clients.get(addr).cloned()
    }
}

#[async_trait]
impl Listener for Perch {
    async fn listen_on(self: Arc<Self>, addr: &str) -> anyhow::Result<()> {
        let addr = addr.to_owned() + ":" + &self.port.to_string();
        let listener = TcpListener::bind(addr).await?;
        slog!("SERVER", "listening on port {}", self.port);

        loop {
            let (mut socket, addr) = listener.accept().await?;
            slog!("SERVER", "new connexion from {:?}", addr);

            let this = Arc::clone(&self);

            tokio::spawn(async move {
                if let Err(e) = this.handle_new_connexion(&mut socket, &addr).await {
                    eprintln!("Error with client {:?}: {:?}", addr, e);
                }
            });
        }
    }

    async fn handle_new_connexion(
        self: Arc<Self>,
        stream: &mut TcpStream,
        addr: &SocketAddr,
    ) -> anyhow::Result<()> {
        let protocol = Protocol::resolve(stream).await?;

        if let Some(handler) = self.handlers.get(&protocol) {
            handler
                .handle(stream, addr, Arc::clone(&self.mock_map))
                .await?;
        } else {
            return Err(anyhow::anyhow!(
                "No handler registred for protocol {:?}",
                protocol
            ));
        }

        Ok(())
    }
}
