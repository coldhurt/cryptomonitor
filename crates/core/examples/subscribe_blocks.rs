//! Example of subscribing to blocks and watching block headers by polling.

use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use eyre::Result;
use futures_util::{stream, StreamExt};

#[tokio::main]
async fn main() -> Result<()> {
    let ws = WsConnect::new("ws://localhost:8545");
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    // Subscribe to block headers.
    let subscription = provider.subscribe_blocks().await?;
    let mut stream = subscription.into_stream().take(2);

    while let Some(header) = stream.next().await {
        println!("Received block number: {}", header.number);
    }

    // Poll for block headers.
    let poller = provider.watch_blocks().await?;
    let mut stream = poller.into_stream().flat_map(stream::iter).take(2);

    let handle = tokio::spawn(async move {
        while let Some(block_hash) = stream.next().await {
            println!("Polled for block header: {block_hash:?}");
        }
    });

    handle.await?;

    Ok(())
}
