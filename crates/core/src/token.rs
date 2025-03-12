use std::collections::HashMap;

use alloy::{
    consensus::Transaction,
    primitives::{Address, address, utils::format_units},
    sol,
    sol_types::SolCall,
};

use crate::swap::print_tx;

sol! {
interface IERC20 {
  function transfer(address to, uint value) public;
}
}
const TETHER: Address = address!("0xdAC17F958D2ee523a2206206994597C13D831ec7");
const USDC: Address = address!("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");
const PEPE: Address = address!("0x6982508145454ce325ddbe47a25d4ec3d2311933");

const TETHER_BASE: Address = address!("0xfde4C96c8593536E31F229EA8f37b2ADa2699bb2");
const USDC_BASE: Address = address!("0x833589fcd6edb6e08f4c7c32d4f71b54bda02913");

const TETHER_BNB: Address = address!("0x55d398326f99059ff775485246999027b3197955");
const USDC_BNB: Address = address!("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d");

fn get_token_addresses(network: &str) -> HashMap<String, (Address, u8)> {
    let token_addresses: HashMap<&str, HashMap<String, (Address, u8)>> = HashMap::from([
        (
            "ethereum",
            HashMap::from([
                ("usdt".to_string(), (TETHER, 6)),
                ("usdc".to_string(), (USDC, 6)),
                ("pepe".to_string(), (PEPE, 18)),
            ]),
        ),
        (
            "base",
            HashMap::from([
                ("usdt".to_string(), (TETHER_BASE, 6)),
                ("usdc".to_string(), (USDC_BASE, 6)),
            ]),
        ),
        (
            "bnb",
            HashMap::from([
                ("usdt".to_string(), (TETHER_BNB, 18)),
                ("usdc".to_string(), (USDC_BNB, 18)),
            ]),
        ),
    ]);

    return token_addresses.get(network).unwrap().clone();
}

pub async fn monitor_tokens(
    network: &str,
    tx: &alloy::rpc::types::Transaction,
    tokens: &Vec<String>,
) {
    let inner = &tx.inner;

    let input = inner.input();
    let to_str = inner.to().unwrap();

    let token_map = get_token_addresses(network);

    for token in tokens.iter() {
        if let Some(&(token_address, decimals)) = token_map.get(token.as_str()) {
            if to_str == token_address {
                if let Ok(decoded) = IERC20::transferCall::abi_decode(&input, true) {
                    println!(
                        "[{} Transfer]\nValue: {:?} => {:?} {}\nFrom: {:?}\nTo: {:?}",
                        token.to_uppercase(),
                        decoded.value,
                        format_units(decoded.value, decimals).unwrap(),
                        token.to_uppercase(),
                        tx.from,
                        decoded.to
                    );
                    print_tx(network, &inner);
                    return;
                }
            }
        }
    }
}
