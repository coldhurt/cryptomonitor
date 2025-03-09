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

pub async fn get_eth_price() -> Result<u64> {
  let ws = WsConnect::new(get_api_url(None));
  let provider = ProviderBuilder::new().on_ws(ws).await?;

  // Create a contract instance.
  let contract = EACAggregatorProxy::new(
      address!("0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419"),
      provider,
  );

  // Call the contract, retrieve the total supply.
  let data = contract.latestRoundData().call().await?;

  // println!("ETH price is {:?}", data.answer.as_u64() / 100_000_000);

  Ok(data.answer.as_u64() / 100_000_000)
}

pub async fn get_token_price(token: String) -> Result<u64, &'static str> {
  if token.to_lowercase() == "eth" {
    return Ok(get_eth_price().await.unwrap());
  }
  
  Err("Not supported this token")
}

pub async fn get_tokens_price(tokens: Vec<String>) {
  for token in tokens.iter() {
    let res= get_token_price(token.clone()).await;
    if res.is_ok() {
      println!("{} : {}", token, res.unwrap());
    } else {
      println!("{} {}", token, res.err().unwrap());
    }
  }
}