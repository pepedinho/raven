use std::sync::Arc;

use raven::listener::{Listener, Perch};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = Arc::new(Perch::new(8080));

    listener.listen_on("0.0.0.0").await?;
    Ok(())
}
