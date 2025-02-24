//! Create new pair by calling uniswap v2 factory contract

use alloy::{
    primitives::address,
    providers::{ProviderBuilder, WsConnect},
    sol,
};
use eyre::Result;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    IUniswapV2Factory,
    "../../abi/IUniswapV2Factory.json"
}

#[tokio::main]
async fn main() -> Result<()> {
    let ws = WsConnect::new("ws://localhost:8545");
    let provider = ProviderBuilder::new().on_ws(ws).await?;
    let factory_address = address!("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f");
    // Create a contract instance.
    let contract = IUniswapV2Factory::new(factory_address, provider);
    let tx: alloy::providers::PendingTransactionBuilder<alloy::network::Ethereum> = contract
        .createPair(
            address!("0x0688eC0D5cB41470B68534FCD289c40Ee6D6724d"),
            address!("0x93C34505F8950022d5C24D14A00865B82576B678"),
        )
        .send()
        .await?;
    println!("Transaction Hash: {:?}", tx.tx_hash());

    Ok(())
}
