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

pub async fn monitor_tokens(tx: &alloy::rpc::types::Transaction, tokens: &Vec<String>) {
    let inner = &tx.inner;

    let input = inner.input();
    let to_str = inner.to().unwrap();

    let has_usdt = tokens.iter().any(|x| x == "usdt");

    if has_usdt {
        match to_str {
            TETHER => {
                if let Ok(decoded) = IERC20::transferCall::abi_decode(&input, true) {
                    println!(
                        "[Tether Transfer]\nValue: {:?} => {:?} USDT\nFrom: {:?}\nTo: {:?}",
                        decoded.value,
                        format_units(decoded.value, 6).unwrap(),
                        tx.from,
                        decoded.to
                    );
                    print_tx(&inner);
                    return;
                }
            }
            _ => {}
        }
    }
}
