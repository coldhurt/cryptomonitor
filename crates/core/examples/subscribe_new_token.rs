use alloy::{
    primitives::address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
    sol,
};
use eyre::Result;
use futures_util::stream::StreamExt;

use utils::get_api_url;

sol! {
  event PairCreated(address indexed token0, address indexed token1, address pair, uint);
}

#[tokio::main]
async fn main() -> Result<()> {
    let ws = WsConnect::new(get_api_url());
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    let uniswap_v3_factory_address = address!("0x1F98431c8aD98523631AE4a59f267346ea31F984");
    let filter_v3 = Filter::new()
      .address(uniswap_v3_factory_address)
    //   .event("PoolCreated(address,address,uint24,int24,address)")
      .from_block(BlockNumberOrTag::Latest);

    // Uniswap V2 Factory - PairCreated
    let uniswap_v2_factory = address!("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f");
    let filter_v2 = Filter::new()
        .address(uniswap_v2_factory)
        // .event("PairCreated(address,address,address,uint256)")
        .from_block(BlockNumberOrTag::Latest);

    // Subscribe to logs.
    let mut stream_v3 = provider.subscribe_logs(&filter_v3).await?.into_stream();
    let mut stream_v2 = provider.subscribe_logs(&filter_v2).await?.into_stream();

    println!("Listening for new token pools...");

    loop {
        tokio::select! {
            Some(log) = stream_v3.next() => {
                println!("🔵 Uniswap V3 New Pool Created: {}", serde_json::to_string_pretty(&log).unwrap());
            }
            Some(log) = stream_v2.next() => {
                println!("🟢 Uniswap V2 New Pair Created: {}", serde_json::to_string_pretty(&log).unwrap());
            }
        }
    }

    // Ok(())
}
