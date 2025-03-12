use crate::swap::print_tx;
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

// BASE
const UNISWAP_V2_FACTORY_BASE: Address = address!("0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6");
const UNISWAP_V3_FACTORY_BASE: Address = address!("0x33128a8fC17869897dcE68Ed026d694621f6FDfD");
const SUSHISWAP_V2_FACTORY_BASE: Address = address!("0x71524B4f93c58fcbF659783284E38825f0622859");

// BNB
const UNISWAP_V2_FACTORY_BNB: Address = address!("0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6");
const UNISWAP_V3_FACTORY_BNB: Address = address!("0xdB1d10011AD0Ff90774D0C6Bb92e5C5c8b4461F7");
const SUSHISWAP_V2_FACTORY_BNB: Address = address!("0xc35DADB65012eC5796536bD9864eD8773aBc74C4");
const PANCAKE_V2_FACTORY_BNB: Address = address!("0xcA143Ce32Fe78f1f7019d7d551a6402fC5350c73");

fn get_dex_addresses(network: &str) -> HashMap<String, Address> {
    let dex_addresses: HashMap<&str, HashMap<String, Address>> = HashMap::from([
        (
            "ethereum",
            HashMap::from([
                ("uniswap".to_string(), UNISWAP_V2_FACTORY),
                ("uniswap_v3".to_string(), UNISWAP_V3_FACTORY),
                ("sushiswap".to_string(), SUSHISWAP_V2_FACTORY),
            ]),
        ),
        (
            "base",
            HashMap::from([
                ("uniswap".to_string(), UNISWAP_V2_FACTORY_BASE),
                ("uniswap_v3".to_string(), UNISWAP_V3_FACTORY_BASE),
                ("sushiswap".to_string(), SUSHISWAP_V2_FACTORY_BASE),
            ]),
        ),
        (
            "bnb",
            HashMap::from([
                ("uniswap".to_string(), UNISWAP_V2_FACTORY_BNB),
                ("uniswap_v3".to_string(), UNISWAP_V3_FACTORY_BNB),
                ("sushiswap".to_string(), SUSHISWAP_V2_FACTORY_BNB),
                ("pancake".to_string(), PANCAKE_V2_FACTORY_BNB),
            ]),
        ),
    ]);

    return dex_addresses.get(network).unwrap().clone();
}

fn v2_pair_transaction(dex_name: &str, inner: &TxEnvelope) {
    let input = inner.input();
    if let Ok(decoded) = UniswapV2Factory::createPairCall::abi_decode(&input, true) {
        println!(
            "[{dex_name} V2 Swap] createPair\nTokenA: {:?}\nTokenB: {:?}",
            decoded.tokenA, decoded.tokenB,
        );
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
        return;
    }
}

async fn analyze_transaction(inner: &TxEnvelope, dex_address: &Address) {
    match dex_address {
        &UNISWAP_V2_FACTORY | &UNISWAP_V2_FACTORY_BASE => {
            v2_pair_transaction("UniSwap", inner);
        }
        &SUSHISWAP_V2_FACTORY => {
            v2_pair_transaction("SushiSwap", inner);
        }
        &UNISWAP_V3_FACTORY | &UNISWAP_V3_FACTORY_BASE => {
            uniswap_v3_pair_transaction(inner);
        }
        _ => {}
    }
}

pub async fn monitor_pairs(network: &str, tx: &alloy::rpc::types::Transaction, dexs: &Vec<String>) {
    let inner = &tx.inner;

    let to_str: Address = inner.to().unwrap();

    let dexs_address = get_dex_addresses(network);

    for dex in dexs.iter() {
        let dex_address = dexs_address.get(dex.as_str()).unwrap();
        if to_str == *dex_address {
            analyze_transaction(inner, &dex_address).await;
            print_tx(network, inner);
            break;
        }
    }
}
