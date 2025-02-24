use std::str::FromStr;

use alloy::{
    primitives::TxHash,
    providers::{Provider, ProviderBuilder, WsConnect},
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let ws = WsConnect::new("ws://localhost:8545");
    let provider = ProviderBuilder::new().on_ws(ws).await?;
  
    let tx_hash =
        TxHash::from_str("0x80e65542f7559825554392200421162132e9f3d3418aa8cfaa6062ee3a30f5b4")?;

    match provider.get_transaction_by_hash(tx_hash).await? {
        Some(tx) => println!("Transaction: {:?}", tx),
        None => println!("Transaction not found"),
    }
    Ok(())
}
