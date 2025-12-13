use std::sync::Arc;

use clap::Parser;
use raven::{
    cli::Cli,
    listener::{Listener, Perch},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let listener = Arc::new(Perch::new(&cli)?);

    println!("Successfully loaded {} mocks", listener.mock_map.len());
    println!("------------------------------------------");

    listener.mock_map.iter().for_each(|(name, mock)| {
        println!("Mock name      : {}", name);
        println!("Type           : {:?}", mock.r#type);
        println!("Match method   : {:?}", mock.r#match.method);
        println!("Match path     : {:?}", mock.r#match.path);
        println!("Match headers  : {:?}", mock.r#match.header);
        println!("Response code  : {}", mock.response.status);
        println!("Response delay : {:?}", mock.response.options.delay);
        println!("------------------------------------------");
    });

    listener.listen_on(&cli.host).await?;
    Ok(())
}
