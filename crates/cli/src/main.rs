use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use eyre::Result;
use futures_util::StreamExt;

use utils::get_api_url;

#[tokio::main]
async fn main() -> Result<()> {
    let ws = WsConnect::new(get_api_url());
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    // Subscribe to new blocks.
    let sub = provider.subscribe_blocks().await?;

    // Wait and take the next 4 blocks.
    let mut stream = sub.into_stream().take(4);

    println!("Awaiting block headers...");

    // Take the stream and print the block number upon receiving a new block.
    let handle = tokio::spawn(async move {
        while let Some(header) = stream.next().await {
            println!("Latest block number: {}", header.number);
        }
    });

    handle.await?;

    Ok(())
}