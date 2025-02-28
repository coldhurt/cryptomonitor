use std::collections::HashMap;

use alloy::{
    consensus::Transaction,
    primitives::{Address, address, utils::format_units},
    sol,
    sol_types::SolCall,
};

use crate::pair::print_tx;

sol! {
interface IERC20 {
  function transfer(address to, uint value) public;
}
}
const TETHER: Address = address!("0xdAC17F958D2ee523a2206206994597C13D831ec7");
const USDC: Address = address!("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");
const PEPE: Address = address!("0x6982508145454ce325ddbe47a25d4ec3d2311933");

pub async fn monitor_tokens(tx: &alloy::rpc::types::Transaction, tokens: &Vec<String>) {
    let inner = &tx.inner;

    let input = inner.input();
    let to_str = inner.to().unwrap();

    let mut token_map = HashMap::new();
    token_map.insert("usdt", (TETHER, 6));
    token_map.insert("usdc", (USDC, 6));
    token_map.insert("pepe", (PEPE, 18));

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
                    print_tx(&inner);
                    return;
                }
            }
        }
    }
}
