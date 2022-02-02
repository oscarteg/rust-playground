mod discord;
mod transmission;

use dotenv::dotenv;
use transmission_rpc::types::Result;

fn transmission() {}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let client = transmission::Transmission::new();

    client.torrent_add().await;
    Ok(())
}
