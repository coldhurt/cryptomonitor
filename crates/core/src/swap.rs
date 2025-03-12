use std::collections::HashMap;

use alloy::{
    consensus::{Transaction, TxEnvelope},
    primitives::{Address, U256, address, utils::format_ether},
    sol,
    sol_types::SolCall,
};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IUniswapV2Router,
    "../../abi/UniswapV2Router.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UniswapV3Router,
    "../../abi/UniswapV3Router.json"
);

// Uniswap router addresses on Ethereum mainnet
const UNISWAP_V2_ROUTER: Address = address!("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D");
const UNISWAP_V3_ROUTER: Address = address!("0xE592427A0AEce92De3Edee1F18E0157C05861564");
const SUSHISWAP_V2_ROUTER: Address = address!("0xd9e1cE17f2641f24aE83637ab66a2cca9C378B9F");
const PANCAKE_V2_ROUTER: Address = address!("0xEfF92A263d31888d860bD50809A8D171709b7b1c");

// Base l2
const UNISWAP_V2_ROUTER_BASE: Address = address!("0x4752ba5dbc23f44d87826276bf6fd6b1c372ad24");
const UNISWAP_V3_ROUTER_BASE: Address = address!("0x2626664c2603336E57B271c5C0b26F421741e481");
const SUSHISWAP_V2_ROUTER_BASE: Address = address!("0x6BDED42c6DA8FBf0d2bA55B2fa120C5e0c8D7891");
const PANCAKE_V2_ROUTER_BASE: Address = address!("0x8cFe327CEc66d1C090Dd72bd0FF11d690C33a2Eb");

// BNB
const UNISWAP_V2_ROUTER_BNB: Address = address!("0x4752ba5dbc23f44d87826276bf6fd6b1c372ad24");
const UNISWAP_V3_ROUTER_BNB: Address = address!("0xB971eF87ede563556b2ED4b1C0b0019111Dd85d2");
const SUSHISWAP_V2_ROUTER_BNB: Address = address!("0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506");
const PANCAKE_V2_ROUTER_BNB: Address = address!("0x10ED43C718714eb63d5aA57B78B54704E256024E");

fn get_dex_addresses(network: &str) -> HashMap<String, Address> {
    let dex_addresses: HashMap<&str, HashMap<String, Address>> = HashMap::from([
        (
            "ethereum",
            HashMap::from([
                ("uniswap".to_string(), UNISWAP_V2_ROUTER),
                ("uniswap_v3".to_string(), UNISWAP_V3_ROUTER),
                ("sushiswap".to_string(), SUSHISWAP_V2_ROUTER),
                ("pancake".to_string(), PANCAKE_V2_ROUTER),
            ]),
        ),
        (
            "base",
            HashMap::from([
                ("uniswap".to_string(), UNISWAP_V2_ROUTER_BASE),
                ("uniswap_v3".to_string(), UNISWAP_V3_ROUTER_BASE),
                ("sushiswap".to_string(), SUSHISWAP_V2_ROUTER_BASE),
                ("pancake".to_string(), PANCAKE_V2_ROUTER_BASE),
            ]),
        ),
        (
            "bnb",
            HashMap::from([
                ("uniswap".to_string(), UNISWAP_V2_ROUTER_BNB),
                ("uniswap_v3".to_string(), UNISWAP_V3_ROUTER_BNB),
                ("sushiswap".to_string(), SUSHISWAP_V2_ROUTER_BNB),
                ("pancake".to_string(), PANCAKE_V2_ROUTER_BNB),
            ]),
        ),
    ]);

    return dex_addresses.get(network).unwrap().clone();
}

pub fn is_valid_dexs(network: &str, dexs: &Vec<String>) -> bool {
    return dexs
        .iter()
        .any(|dex| get_dex_addresses(network).contains_key(dex.as_str()));
}

pub fn print_tx(network: &str, tx: &TxEnvelope) {
    let explorer_domains = HashMap::from([
        ("ethereum", "https://etherscan.io/tx/"),
        ("base", "https://basescan.org/tx/"),
        ("bnb", "https://bscscan.com/tx/"),
    ]);
    println!(
        "Tx: {}{}",
        explorer_domains.get(network).unwrap(),
        tx.tx_hash()
    );
}

fn v2_swap_transaction(dex_name: &str, inner: &TxEnvelope) {
    let input = inner.input();
    let value = inner.value();
    if value.gt(&U256::from(0)) {
        if let Ok(decoded) = IUniswapV2Router::swapETHForExactTokensCall::abi_decode(&input, true) {
            println!(
                "[{dex_name} V2 Swap] swapETHForExactTokens\nValue: {:?}ETH\nAmountOut: {:?}\nPath: {:?}",
                format_ether(value),
                decoded.amountOut,
                decoded.path
            );
            return;
        } else if let Ok(decoded) =
            IUniswapV2Router::swapExactETHForTokensCall::abi_decode(&input, true)
        {
            println!(
                "[{dex_name} V2 Swap] swapExactETHForTokens\nValue: {:?}ETH\nAmountOutMin: {:?}\nPath: {:?}",
                format_ether(value),
                decoded.amountOutMin,
                decoded.path
            );
            return;
        }
    } else if let Ok(decoded) =
        IUniswapV2Router::swapExactTokensForTokensCall::abi_decode(&input, true)
    {
        println!(
            "[{dex_name} V2 Swap] swapExactTokensForTokensCall\nAmountIn: {:?}\nPath: {:?}",
            decoded.amountIn, decoded.path
        );
        return;
    } else if let Ok(decoded) =
        IUniswapV2Router::swapTokensForExactTokensCall::abi_decode(&input, true)
    {
        println!(
            "[{dex_name} V2 Swap] swapTokensForExactTokensCall\namountInMax: {:?}\nPath: {:?}",
            decoded.amountInMax, decoded.path
        );
        return;
    } else if let Ok(decoded) =
        IUniswapV2Router::swapExactTokensForETHCall::abi_decode(&input, true)
    {
        println!(
            "[{dex_name} V2 Swap] swapExactTokensForETHCall\nAmountIn: {:?}\nPath: {:?}",
            decoded.amountIn, decoded.path
        );
        return;
    } else if let Ok(decoded) =
        IUniswapV2Router::swapTokensForExactETHCall::abi_decode(&input, true)
    {
        println!(
            "[{dex_name} V2 Swap] swapTokensForExactETHCall\namountInMax: {:?}\nPath: {:?}",
            decoded.amountInMax, decoded.path
        );
        return;
    } else if let Ok(decoded) =
        IUniswapV2Router::swapExactETHForTokensSupportingFeeOnTransferTokensCall::abi_decode(
            &input, true,
        )
    {
        println!(
            "[{dex_name} V2 Swap] swapExactETHForTokensSupportingFeeOnTransferTokensCall\namountOutMin: {:?}\nPath: {:?}",
            decoded.amountOutMin, decoded.path
        );
        return;
    } else if let Ok(decoded) =
        IUniswapV2Router::swapExactTokensForETHSupportingFeeOnTransferTokensCall::abi_decode(
            &input, true,
        )
    {
        println!(
            "[{dex_name} V2 Swap] swapExactTokensForETHSupportingFeeOnTransferTokensCall\namountOutMin: {:?}\nPath: {:?}",
            decoded.amountOutMin, decoded.path
        );
        return;
    } else if let Ok(decoded) =
        IUniswapV2Router::swapExactTokensForTokensSupportingFeeOnTransferTokensCall::abi_decode(
            &input, true,
        )
    {
        println!(
            "[{dex_name} V2 Swap] swapExactTokensForTokensSupportingFeeOnTransferTokensCall\namountOutMin: {:?}\nPath: {:?}",
            decoded.amountOutMin, decoded.path
        );
        return;
    }
}

fn uniswap_v3_swap_transaction(inner: &TxEnvelope) {
    let input = inner.input();
    if let Ok(decoded) = UniswapV3Router::exactInputCall::abi_decode(&input, true) {
        println!(
            "[UniSwap V3 Swap] exactInput\nAmountIn: {:?}\nPath: {:?}",
            decoded.params.amountIn, decoded.params.path
        );
        return;
    } else if let Ok(decoded) = UniswapV3Router::exactInputSingleCall::abi_decode(&input, true) {
        println!(
            "[UniSwap V3 Swap] exactInputSingle\nAmountIn: {:?}\nTokenIn: {:?}\nTokenOut: {:?}",
            decoded.params.amountIn, decoded.params.tokenIn, decoded.params.tokenOut,
        );
        return;
    } else if let Ok(decoded) = UniswapV3Router::exactOutputSingleCall::abi_decode(&input, true) {
        println!(
            "[UniSwap V3 Swap] exactOutputSingle\namountInMaximum: {:?}\nTokenIn: {:?}\nTokenOut: {:?}",
            decoded.params.amountInMaximum, decoded.params.tokenIn, decoded.params.tokenOut,
        );
        return;
    } else if let Ok(decoded) = UniswapV3Router::exactOutputCall::abi_decode(&input, true) {
        println!(
            "[UniSwap V3 Swap] exactOutput\namountInMaximum: {:?}\nPath: {:?}",
            decoded.params.amountInMaximum, decoded.params.path,
        );
        return;
    }
}

async fn analyze_transaction(inner: &TxEnvelope, dex_address: &Address) {
    match dex_address {
        &UNISWAP_V2_ROUTER | &UNISWAP_V2_ROUTER_BASE => {
            v2_swap_transaction("UniSwap", inner);
        }
        &SUSHISWAP_V2_ROUTER | &SUSHISWAP_V2_ROUTER_BASE | &SUSHISWAP_V2_ROUTER_BNB => {
            v2_swap_transaction("SushiSwap", inner);
        }
        &PANCAKE_V2_ROUTER | &PANCAKE_V2_ROUTER_BASE | &PANCAKE_V2_ROUTER_BNB => {
            v2_swap_transaction("PancakeSwap", inner);
        }
        &UNISWAP_V3_ROUTER | &UNISWAP_V3_ROUTER_BASE | &UNISWAP_V3_ROUTER_BNB => {
            uniswap_v3_swap_transaction(inner);
        }
        _ => {}
    }
}

pub async fn monitor_swaps(network: &str, tx: &alloy::rpc::types::Transaction, dexs: &Vec<String>) {
    let inner = &tx.inner;

    let to_str: Address = inner.to().unwrap();

    let dexs_address = get_dex_addresses(network);

    for dex in dexs.iter() {
        let dex_address = dexs_address.get(dex.as_str()).unwrap();
        if to_str == *dex_address {
            analyze_transaction(inner, &dex_address).await;
            print_tx(network, &inner);
            break;
        }
    }
}
