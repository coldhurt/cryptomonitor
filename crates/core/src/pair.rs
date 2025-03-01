use std::collections::HashMap;

use alloy::{
    consensus::{Transaction, TxEnvelope},
    primitives::{Address, address},
    sol,
    sol_types::SolCall,
};

sol!(
  #[allow(missing_docs)]
  #[sol(rpc)]
  UniswapV2Factory,
  "../../abi/UniswapV2Factory.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UniswapV3Factory,
    "../../abi/UniswapV3Factory.json"
);

// Uniswap factory addresses on Ethereum mainnet
const UNISWAP_V2_FACTORY: Address = address!("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f");
const UNISWAP_V3_FACTORY: Address = address!("0x1F98431c8aD98523631AE4a59f267346ea31F984");
const SUSHISWAP_V2_FACTORY: Address = address!("0xc0aee478e3658e2610c5f7a4a2e1777ce9e4f2ac");

fn get_dex_addresses() -> HashMap<&'static str, Address> {
    HashMap::from([
        ("uniswap", UNISWAP_V2_FACTORY),
        ("uniswap_v3", UNISWAP_V3_FACTORY),
        ("sushiswap", SUSHISWAP_V2_FACTORY),
    ])
}

pub fn print_tx(tx: &TxEnvelope) {
    println!("Tx: https://etherscan.io/tx/{}", tx.tx_hash(),);
}

fn v2_pair_transaction(dex_name: &str, inner: &TxEnvelope) {
    let input = inner.input();
    if let Ok(decoded) = UniswapV2Factory::createPairCall::abi_decode(&input, true) {
        println!(
            "[{dex_name} V2 Swap] createPair\nTokenA: {:?}\nTokenB: {:?}",
            decoded.tokenA,
            decoded.tokenB,
        );
        print_tx(&inner);
        return;
    } 
}

fn uniswap_v3_pair_transaction(inner: &TxEnvelope) {
    let input = inner.input();
    if let Ok(decoded) = UniswapV3Factory::createPoolCall::abi_decode(&input, true) {
        println!(
            "[UniSwap V3 Swap] createPool\nTokenA: {:?}\nTokenB: {:?}",
            decoded.tokenA, decoded.tokenB
        );
        print_tx(&inner);
        return;
    }
}

async fn analyze_transaction(inner: &TxEnvelope, dex_address: &Address) {
    match dex_address {
        &UNISWAP_V2_FACTORY => {
            v2_pair_transaction("UniSwap", inner);
        }
        &SUSHISWAP_V2_FACTORY => {
            v2_pair_transaction("SushiSwap", inner);
        }
        &UNISWAP_V3_FACTORY => {
            uniswap_v3_pair_transaction(inner);
        }
        _ => {}
    }
}

pub async fn monitor_pairs(tx: &alloy::rpc::types::Transaction, dexs: &Vec<String>) {
    let inner = &tx.inner;

    let to_str: Address = inner.to().unwrap();

    let dexs_address = get_dex_addresses();

    for dex in dexs.iter() {
        let dex_address = dexs_address.get(dex.as_str()).unwrap();
        if to_str == *dex_address {
            analyze_transaction(inner, &dex_address).await;
            break;
        }
    }
}
