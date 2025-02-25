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
    MyNFT,
    "../../contracts/out/MyNFT.sol/MyNFT.json"
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

    let builder = MyNFT::deploy_builder(&provider).from(alice);

    let contract_address = builder.deploy().await?;
    println!("Deployed contract at address: {}", contract_address);

    let contract = MyNFT::new(contract_address, provider);
    let name = contract.name().call().await.unwrap()._0;
    let symbol = contract.symbol().call().await.unwrap()._0;
    println!("NFT name: {name}\nSymbol: {symbol}\n");

    let balance_of_alice = contract.balanceOf(alice).call().await.unwrap()._0;
    println!("Initial balance of alice: {balance_of_alice}");

    let mint_tx = contract
        .mintNFT(alice, "https://myt0.com/MNFT/URI0.png".to_string())
        .from(alice)
        .send()
        .await
        .unwrap();

    println!("Mint tx: {}", mint_tx.tx_hash());

    let balance_of_alice = contract.balanceOf(alice).call().await.unwrap()._0;
    println!("Balance of alice: {balance_of_alice}");

    let token_id = U256::from(0);
    let uri_0 = contract.tokenURI(token_id).call().await.unwrap()._0;
    println!("URI of 0: {uri_0}");
    let owner_0 = contract.ownerOf(token_id).call().await.unwrap()._0;
    println!("Owner of 0: {owner_0}");
    let transfer_tx = contract
        .transferFrom(alice, bob, token_id)
        .from(alice)
        .send()
        .await
        .unwrap();
    println!(
        "Alice transfer the token {token_id} to bob, tx is {}",
        transfer_tx.tx_hash()
    );
    let owner_0 = contract.ownerOf(token_id).call().await.unwrap()._0;
    println!("After the transfering, the owner of 0: {owner_0}");

    // This won't succeed because bob is not the owner, he can't mint token
    // let mint_tx_from_bob = contract
    //     .mintNFT(alice, "URI2".to_string())
    //     .from(bob)
    //     .send()
    //     .await
    //     .expect("Mint failed");
    // println!("Mint tx from bob: {}", mint_tx_from_bob.tx_hash());
    Ok(())
}
