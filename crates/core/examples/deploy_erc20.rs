//! Example of deploying a contract from an artifact using the `sol!` macro to Anvil and interacting
//! with it.

use alloy::{
    primitives::{
        U256,
        utils::{format_units, parse_units},
    },
    providers::{Provider, ProviderBuilder, WsConnect},
    sol,
};
use eyre::Result;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    MyToken,
    "../../contracts/out/MyToken.sol/MyToken.json"
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
    let bob = accounts[1];

    let builder = MyToken::deploy_builder(&provider).from(alice);

    let contract_address = builder.deploy().await?;
    println!("Deployed contract at address: {}", contract_address);

    let contract = MyToken::new(contract_address, provider);
    let total_supply = contract.totalSupply().call().await.unwrap()._0;
    let decimals = contract.decimals().call().await.unwrap()._0;
    let name = contract.name().call().await.unwrap()._0;
    let symbol = contract.symbol().call().await.unwrap()._0;
    println!(
        "Token name: {name}\nSymbol: {symbol}\nTotal supply: {total_supply}\nDecimals: {decimals}"
    );

    let balance_of_alice = contract.balanceOf(alice).call().await.unwrap()._0;

    println!(
        "Balance of alice: {} {symbol}",
        format_units(balance_of_alice, decimals).unwrap()
    );

    let amount = parse_units("100", decimals).unwrap().into();
    println!("Alice transfer to bob {}", amount);
    let transfer_to_bob = contract
        .transfer(bob, amount)
        .from(alice)
        .send()
        .await
        .unwrap();
    println!("Transfer tx hash: {}", transfer_to_bob.tx_hash());
    let balance_of_alice = contract.balanceOf(alice).call().await.unwrap()._0;
    let balance_of_bob = contract.balanceOf(bob).call().await.unwrap()._0;
    println!(
        "Balance of alice: {} {symbol}",
        format_units(balance_of_alice, decimals).unwrap()
    );
    println!(
        "Balance of bob: {} {symbol}",
        format_units(balance_of_bob, decimals).unwrap()
    );
    Ok(())
}
