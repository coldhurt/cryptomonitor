//! Example of deploying a contract from an artifact using the `sol!` macro to Anvil and interacting
//! with it.

use alloy::{
    primitives::{Bytes, U256},
    providers::{Provider, ProviderBuilder, WsConnect},
    sol,
};
use eyre::Result;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    MyERC1155,
    "../../contracts/out/MyERC1155.sol/MyERC1155.json"
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

    let builder = MyERC1155::deploy_builder(&provider).from(alice);

    let contract_address = builder.deploy().await?;
    println!("Deployed contract at address: {}", contract_address);

    let contract = MyERC1155::new(contract_address, provider);

    let token_id = U256::from(888);
    let token_id2 = U256::from(999);
    let mint_amount = U256::from(100);

    println!("Mint token_{token_id}=>{mint_amount} token_{token_id2}=>{mint_amount}");
    let mint_tx = contract
        .mintBatch(
            alice,
            vec![token_id, token_id2],
            vec![mint_amount, mint_amount],
        )
        .from(alice)
        .send()
        .await
        .unwrap();
    println!("Mint batch tx: {}", mint_tx.tx_hash());

    // println!("Mint token_{token_id} {mint_amount}");
    // let mint_tx = contract
    //     .mint(alice, token_id, mint_amount)
    //     .from(alice)
    //     .send()
    //     .await
    //     .unwrap();
    // println!("Mint {token_id} tx: {}", mint_tx.tx_hash());

    // println!("Mint token_{token_id2} {mint_amount}");
    // let mint_tx = contract
    //     .mint(alice, token_id2, mint_amount)
    //     .from(alice)
    //     .send()
    //     .await
    //     .unwrap();
    // println!("Mint {token_id2} tx: {}", mint_tx.tx_hash());

    // let balance_of_alice = contract.balanceOf(alice, token_id).call().await.unwrap()._0;
    // println!("Balance token_{token_id} of Alice : {balance_of_alice}");

    let balance_batchs = contract
        .balanceOfBatch(vec![alice, alice], vec![token_id, token_id2])
        .call()
        .await
        .unwrap();
    println!(
        "Balance of Alice {token_id}=>{} {token_id2}=>{}",
        balance_batchs._0[0], balance_batchs._0[1]
    );

    let transfer_tx = contract
        .safeBatchTransferFrom(
            alice,
            bob,
            vec![token_id, token_id2],
            vec![U256::from(10), U256::from(20)],
            Bytes::from(""),
        )
        .from(alice)
        .send()
        .await
        .unwrap();

    println!("Transfer tx: {}", transfer_tx.tx_hash());

    let balance_batchs = contract
        .balanceOfBatch(
            vec![alice, alice, bob, bob],
            vec![token_id, token_id2, token_id, token_id2],
        )
        .call()
        .await
        .unwrap();

    println!(
        "Balance of Alice {token_id}=>{} {token_id2}=>{}",
        balance_batchs._0[0], balance_batchs._0[1]
    );

    println!(
        "Balance of Bob {token_id}=>{} {token_id2}=>{}",
        balance_batchs._0[2], balance_batchs._0[3]
    );

    Ok(())
}
