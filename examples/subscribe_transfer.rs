//! Example of subscribing and listening for specific contract events by `WebSocket` subscription.

use alloy::{
  primitives::address,
  providers::{Provider, ProviderBuilder, WsConnect},
  rpc::types::{BlockNumberOrTag, Filter},
};
use eyre::Result;
use futures_util::stream::StreamExt;
use utils::get_api_url;

#[path = "../src/utils/mod.rs"]
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
  // Create the provider.
  let ws = WsConnect::new(get_api_url());
  let provider = ProviderBuilder::new().on_ws(ws).await?;

  let uniswap_token_address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");
  let filter = Filter::new()
      .address(uniswap_token_address)
      .event("Transfer(address,address,uint256)")
      .from_block(BlockNumberOrTag::Latest);

  // Subscribe to logs.
  let sub = provider.subscribe_logs(&filter).await?;
  let mut stream = sub.into_stream();

  while let Some(log) = stream.next().await {
      println!("Uniswap token logs: {log:?}");
  }

  Ok(())
}