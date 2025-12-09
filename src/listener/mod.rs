use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use tokio::net::{TcpListener, TcpStream};

use crate::resolver::Protocol;

#[async_trait]
pub trait Listener: Send + Sync + 'static {
    async fn listen_on(self: Arc<Self>, addr: &str) -> anyhow::Result<()>;
    async fn handle_new_connexion(self: Arc<Self>, stream: &mut TcpStream) -> anyhow::Result<()>;
}

#[derive(Clone)]
pub struct Client {}

/// Perch is concretly the Listener we use to Listen clients requests
pub struct Perch {
    port: u16,
    clients: HashMap<String, Client>,
}

impl Perch {
    pub fn check_client(&self, addr: &str) -> Option<Client> {
        self.clients.get(addr).cloned()
    }
}

#[async_trait]
impl Listener for Perch {
    async fn listen_on(self: Arc<Self>, addr: &str) -> anyhow::Result<()> {
        let listener = TcpListener::bind(addr).await?;
        println!("listening started, ready to accept");

        loop {
            let (mut socket, addr) = listener.accept().await?;
            println!("new connexion from {:?}", addr);

            let this = Arc::clone(&self);

            tokio::spawn(async move {
                if let Err(e) = this.handle_new_connexion(&mut socket).await {
                    eprintln!("Error with client {:?}: {:?}", addr, e);
                }
            });
        }
    }

    async fn handle_new_connexion(self: Arc<Self>, stream: &mut TcpStream) -> anyhow::Result<()> {
        match Protocol::resolve(stream).await? {
            Protocol::Http => {}
            Protocol::WebSocket => {}
            Protocol::Custom => {}
        }

        Ok(())
    }
}
