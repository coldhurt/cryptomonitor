use alloy::{
    primitives::address,
    providers::{ProviderBuilder, WsConnect},
    sol,
};
use eyre::Result;
use utils::get_api_url;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    EACAggregatorProxy,
    "../../abi/EACAggregatorProxy.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    let ws = WsConnect::new(get_api_url());
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    // Create a contract instance.
    let contract = EACAggregatorProxy::new(
        address!("0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419"),
        provider,
    );

    // Call the contract, retrieve the total supply.
    let data = contract.latestRoundData().call().await?;

    println!("ETH price is {:?}", data.answer.as_u64() / 100_000_000);

    Ok(())
}
