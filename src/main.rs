use raven::server::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = Server::new("../script/raven.lua".to_string()).await?;

    tokio::spawn(async move {
        server.start().await.unwrap();
    });

    Ok(())
}
