use std::collections::HashMap;

use alloy::{
    consensus::{Transaction, TxEnvelope},
    primitives::{Address, U256, address, utils::format_ether},
    sol,
    sol_types::SolCall,
};

sol! {
// Uniswap V2 Router
interface IUniswapV2Router {
    function swapExactTokensForTokens(
        uint256 amountIn,
        uint256 amountOutMin,
        address[] path,
        address to,
        uint256 deadline
    ) external;
    function swapTokensForExactTokens(
        uint amountOut,
        uint amountInMax,
        address[] calldata path,
        address to,
        uint deadline
    ) external returns (uint[] memory amounts);
    function swapTokensForExactETH(uint amountOut, uint amountInMax, address[] calldata path, address to, uint deadline)
        external
        returns (uint[] memory amounts);
    function swapExactTokensForETH(uint amountIn, uint amountOutMin, address[] calldata path, address to, uint deadline)
        external
        returns (uint[] memory amounts);
    function swapETHForExactTokens(uint amountOut, address[] calldata path, address to, uint deadline)
        external
        payable
        returns (uint[] memory amounts);
    function swapExactETHForTokens(uint amountOutMin, address[] calldata path, address to, uint deadline)
        external
        virtual
        override
        payable
        ensure(deadline)
        returns (uint[] memory amounts);
}

// Uniswap V3 Router
interface IUniswapV3Router {
    struct ExactInputParams {
        bytes path;
        address recipient;
        uint256 deadline;
        uint256 amountIn;
        uint256 amountOutMinimum;
    }

    function exactInput(ExactInputParams calldata params) external payable;
}
}

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UniswapV3Router,
    "../../abi/UniswapV3Router.json"
);

// Uniswap router addresses on Ethereum mainnet
const UNISWAP_V2_ROUTER: Address = address!("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D");
const UNISWAP_V3_ROUTER: Address = address!("0xE592427A0AEce92De3Edee1F18E0157C05861564");
const SUSHISWAP_V2_ROUTER: Address = address!("0x85CD07Ea01423b1E937929B44E4Ad8c40BbB5E71");

fn get_dex_addresses() -> HashMap<&'static str, Address> {
    HashMap::from([
        ("uniswap", UNISWAP_V2_ROUTER),
        ("uniswap_v3", UNISWAP_V3_ROUTER),
        ("sushiswap", SUSHISWAP_V2_ROUTER),
    ])
}

pub fn is_valid_dexs(dexs: &Vec<String>) -> bool {
    return dexs
    .iter()
    .any(|dex| get_dex_addresses().contains_key(dex.as_str()));
}

pub fn print_tx(tx: &TxEnvelope) {
    println!("Tx: https://etherscan.io/tx/{}", tx.tx_hash(),);
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
            print_tx(&inner);
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
            print_tx(&inner);
            return;
        }
    } else if let Ok(decoded) =
        IUniswapV2Router::swapExactTokensForTokensCall::abi_decode(&input, true)
    {
        println!(
            "[{dex_name} V2 Swap] swapExactTokensForTokensCall\nAmountIn: {:?}\nPath: {:?}",
            decoded.amountIn, decoded.path
        );
        print_tx(&inner);
        return;
    } else if let Ok(decoded) =
        IUniswapV2Router::swapTokensForExactTokensCall::abi_decode(&input, true)
    {
        println!(
            "[{dex_name} V2 Swap] swapTokensForExactTokensCall\namountInMax: {:?}\nPath: {:?}",
            decoded.amountInMax, decoded.path
        );
        print_tx(&inner);
        return;
    } else if let Ok(decoded) =
        IUniswapV2Router::swapExactTokensForETHCall::abi_decode(&input, true)
    {
        println!(
            "[{dex_name} V2 Swap] swapExactTokensForETHCall\nAmountIn: {:?}\nPath: {:?}",
            decoded.amountIn, decoded.path
        );
        print_tx(&inner);
        return;
    } else if let Ok(decoded) =
        IUniswapV2Router::swapTokensForExactETHCall::abi_decode(&input, true)
    {
        println!(
            "[{dex_name} V2 Swap] swapTokensForExactETHCall\namountInMax: {:?}\nPath: {:?}",
            decoded.amountInMax, decoded.path
        );
        print_tx(&inner);
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
        print_tx(&inner);
        return;
    } else if let Ok(decoded) = UniswapV3Router::exactInputSingleCall::abi_decode(&input, true) {
        println!(
            "[UniSwap V3 Swap] exactInputSingle\nAmountIn: {:?}\nTokenIn: {:?}\nTokenOut: {:?}",
            decoded.params.amountIn, decoded.params.tokenIn, decoded.params.tokenOut,
        );
        print_tx(&inner);
        return;
    } else if let Ok(decoded) = UniswapV3Router::exactOutputSingleCall::abi_decode(&input, true) {
        println!(
            "[UniSwap V3 Swap] exactOutputSingle\namountInMaximum: {:?}\nTokenIn: {:?}\nTokenOut: {:?}",
            decoded.params.amountInMaximum, decoded.params.tokenIn, decoded.params.tokenOut,
        );
        print_tx(&inner);
        return;
    } else if let Ok(decoded) = UniswapV3Router::exactOutputCall::abi_decode(&input, true) {
        println!(
            "[UniSwap V3 Swap] exactOutput\namountInMaximum: {:?}\nPath: {:?}",
            decoded.params.amountInMaximum, decoded.params.path,
        );
        print_tx(&inner);
        return;
    } 
}

async fn analyze_transaction(inner: &TxEnvelope, dex_address: &Address) {
    match dex_address {
        &UNISWAP_V2_ROUTER => {
            v2_swap_transaction("UniSwap", inner);
        }
        &SUSHISWAP_V2_ROUTER => {
            v2_swap_transaction("SushiSwap", inner);
        }
        &UNISWAP_V3_ROUTER => {
            uniswap_v3_swap_transaction(inner);
        }
        _ => {}
    }
}

pub async fn monitor_swaps(tx: &alloy::rpc::types::Transaction, dexs: &Vec<String>) {
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
