use std::collections::HashMap;

use alloy::{
    primitives::{Address, address},
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

pub fn get_price_contract_addresses() -> HashMap<String, Address> {
    let map = HashMap::from([
        (
            "ETH".to_string(),
            address!("0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419"),
        ),
        (
            "UNI".to_string(),
            address!("0x553303d460EE0afB37EdFf9bE42922D8FF63220e"),
        ),
        (
            "BTC".to_string(),
            address!("0xF4030086522a5bEEa4988F8cA5B36dbC97BeE88c"),
        ),
        (
            "BNB".to_string(),
            address!("0x14e613AC84a31f709eadbdF89C6CC390fDc9540A"),
        ),
        (
            "SOL".to_string(),
            address!("0x4ffC43a60e009B551865A93d232E33Fce9f01507"),
        ),
    ]);
    return map;
}

pub async fn get_token_price(token: &String) -> Result<f64> {
    let ws = WsConnect::new(get_api_url(None));
    let provider = ProviderBuilder::new().on_ws(ws).await?;
    let addresses = get_price_contract_addresses();
    let addr = addresses.get(token);
    match addr {
        Some(addr) => {
            // Create a contract instance.
            let contract = EACAggregatorProxy::new(*addr, provider);

            let data = contract.latestRoundData().call().await?;
            let answer = data.answer.as_u64();
            Ok((answer as f64) / 100_000_000.0)
        }
        _ => Ok(0.0),
    }
}

pub async fn get_tokens_price(tokens: &Vec<String>) {
    for token in tokens.iter() {
        let token_name = token.to_uppercase();
        let res = get_token_price(&token_name).await;
        match res {
            Ok(price) => {
                println!("{} : {}", token_name, price);
            }
            Err(err) => {
                println!("{} {}", token_name, err);
            }
        }
    }
}
