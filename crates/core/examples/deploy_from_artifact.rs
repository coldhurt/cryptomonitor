//! Example of deploying a contract from an artifact using the `sol!` macro to Anvil and interacting
//! with it.

use alloy::{
    primitives::U256,
    providers::{Provider, ProviderBuilder, WsConnect},
    sol,
};
use eyre::Result;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Counter,
    "../../contracts/out/Counter.sol/Counter.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    // let provider = ProviderBuilder::new().on_anvil_with_wallet();
    let ws = WsConnect::new("ws://localhost:8545");
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    let accounts = provider.get_accounts().await.unwrap();
    let alice = accounts[0];

    let builder = Counter::deploy_builder(&provider).from(alice);

    let contract_address = builder.deploy().await?;
    println!("Deployed contract at address: {}", contract_address);

    // Deploy the `Counter` contract.
    let contract = Counter::new(contract_address, provider);

    // Set the number to 42.
    let builder = contract.setNumber(U256::from(42)).from(alice);
    let tx_hash = builder.send().await?.watch().await?;

    println!("Set number to 42: {tx_hash}");

    // Increment the number to 43.
    let builder = contract.increment().from(alice);
    let tx_hash = builder.send().await?.watch().await?;

    println!("Incremented number: {tx_hash}");

    // Retrieve the number, which should be 43.
    let builder = contract.number();

    let number = builder.call().await?._0;

    println!("Retrieved number: {number}");

    Ok(())
}
