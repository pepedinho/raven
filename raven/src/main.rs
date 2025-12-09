use std::sync::Arc;

use clap::Parser;
use raven::{
    cli::Cli,
    listener::{Listener, Perch},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mock_map = match (&cli.mock, &cli.mocks) {
        (Some(file), _) => Cli::load_mock_file(file)?,
        (_, Some(dir)) => Cli::load_mock_dir(dir)?,
        _ => {
            return Err(anyhow::anyhow!(
                "You must provide etiher --mock <file> or --mocks <dir>"
            ));
        }
    };

    // let content = std::fs::read_to_string("test.toml")?;
    // let parsed: MockDefinition = toml::from_str(&content)?;

    println!("Successfully loaded {} mocks", mock_map.len());
    println!("------------------------------------------");

    mock_map.iter().for_each(|(name, mock)| {
        println!("Mock name      : {}", name);
        println!("Type           : {:?}", mock.r#type);
        println!("Match method   : {:?}", mock.r#match.method);
        println!("Match path     : {:?}", mock.r#match.path);
        println!("Match headers  : {:?}", mock.r#match.header);
        println!("Response code  : {}", mock.response.status);
        println!("Response delay : {:?}", mock.response.options.delay);
        println!("------------------------------------------");
    });

    let listener = Arc::new(Perch::new(cli.port));

    listener.listen_on(&cli.host).await?;
    Ok(())
}
